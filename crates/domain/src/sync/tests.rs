use crate::{
    assets::registry::AssetRegistry,
    events::EventBus,
    media::{
        discovered::{DiscoveredMedia, DiscoveredMediaMetadata, GameMetadata},
        types::MediaType,
    },
    metadata::{
        models::{GeneralMetadata, MetadataProviderId, MetadataQuery},
        registry::MetadataRegistry,
        traits::{GeneralInfoFetcher, MetadataFetcher},
    },
    storefronts::{
        error::StorefrontError,
        models::StorefrontId,
        registry::StorefrontRegistry,
        traits::{GameLibraryProvider, Storefront},
    },
    sync::LibrarySyncService,
};
use async_trait::async_trait;
use itonda_database::{media::find_media_by_title, test_utils::setup_db};
use std::sync::Arc;

struct FakeSteamStorefront {
    games: Vec<DiscoveredMedia>,
}

impl FakeSteamStorefront {
    fn new(games: Vec<DiscoveredMedia>) -> Self {
        Self { games }
    }
}

impl Storefront for FakeSteamStorefront {
    fn id(&self) -> StorefrontId {
        StorefrontId::Steam
    }

    fn name(&self) -> &'static str {
        "Steam"
    }
}

#[async_trait]
impl GameLibraryProvider for FakeSteamStorefront {
    async fn owned_games(&self) -> Result<Vec<DiscoveredMedia>, StorefrontError> {
        Ok(self.games.clone())
    }
}

pub fn discovered_game(title: &str) -> DiscoveredMedia {
    DiscoveredMedia {
        title: title.to_string(),
        media_type: MediaType::Game,
        storefront: StorefrontId::Steam,
        external_id: format!("ext-{}", title),
        metadata: DiscoveredMediaMetadata::Game(GameMetadata {
            total_playtime: None,
            last_played: None,
        }),
        launch: None,
    }
}

fn test_storefront_registry(storefront: Arc<dyn GameLibraryProvider>) -> StorefrontRegistry {
    let registry = StorefrontRegistry::new();

    registry.register(storefront);

    registry
}

#[tokio::test]
async fn syncs_storefront_games() {
    let pool = setup_db().await;

    let media = find_media_by_title(&pool, "Portal 2".into()).await.unwrap();

    assert!(media.is_none());

    let storefronts =
        test_storefront_registry(Arc::new(FakeSteamStorefront::new(vec![discovered_game(
            "Portal 2",
        )])));
    let events = EventBus::new();
    let assets = AssetRegistry::new();
    let metadata = MetadataRegistry::new();

    let service = LibrarySyncService::new(
        uuid::Uuid::new_v4(),
        pool.clone(),
        events,
        crate::agents::AgentManager::new(),
        storefronts,
        assets,
        metadata,
    );

    service.sync_all(false).await.unwrap();

    let media = find_media_by_title(&pool, "Portal 2".into()).await.unwrap();

    assert!(media.is_some());
}

#[tokio::test]
async fn syncs_existing_db_media_items() {
    use crate::media::types::MediaStatus;
    use itonda_database::media::{MediaInsert, insert_media};

    let pool = setup_db().await;

    let media_row = insert_media(
        &pool,
        MediaInsert {
            title: "Mr. Robot".into(),
            media_type: "tv_show".into(),
            status_id: MediaStatus::NotStarted.id(),
            ..Default::default()
        },
    )
    .await
    .unwrap();

    let storefronts = StorefrontRegistry::new();
    let events = EventBus::new();
    let assets = AssetRegistry::new();
    let metadata = MetadataRegistry::new();

    let service = LibrarySyncService::new(
        uuid::Uuid::new_v4(),
        pool.clone(),
        events,
        crate::agents::AgentManager::new(),
        storefronts,
        assets,
        metadata,
    );

    service.sync_all(false).await.unwrap();

    let media = find_media_by_title(&pool, "Mr. Robot".into())
        .await
        .unwrap();
    assert!(media.is_some());
    assert_eq!(media.unwrap().id, media_row.id);
}

#[tokio::test]
async fn sync_all_triggers_agent_scan() {
    use crate::protocol::ServerToAgentMessage;
    use tokio::sync::mpsc;

    let pool = setup_db().await;
    let agents = crate::agents::AgentManager::new();
    let (tx, mut rx) = mpsc::channel(10);
    agents.register("agent-123".into(), tx).await;

    let storefronts = StorefrontRegistry::new();
    let events = EventBus::new();
    let assets = AssetRegistry::new();
    let metadata = MetadataRegistry::new();

    let service = LibrarySyncService::new(
        uuid::Uuid::new_v4(),
        pool.clone(),
        events,
        agents,
        storefronts,
        assets,
        metadata,
    );

    service.sync_all(false).await.unwrap();

    let received = rx.recv().await;
    assert!(matches!(received, Some(ServerToAgentMessage::Scan(_))));
}

#[tokio::test]
async fn sync_all_continues_when_item_fails() {
    let pool = setup_db().await;

    let storefronts = test_storefront_registry(Arc::new(FakeSteamStorefront::new(vec![
        discovered_game("Game 1"),
        discovered_game("Game 2"),
        discovered_game("Game 3"),
    ])));
    let events = EventBus::new();
    let assets = AssetRegistry::new();
    let metadata = MetadataRegistry::new();

    let service = LibrarySyncService::new(
        uuid::Uuid::new_v4(),
        pool.clone(),
        events,
        crate::agents::AgentManager::new(),
        storefronts,
        assets,
        metadata,
    );

    service.sync_all(false).await.unwrap();

    let media1 = find_media_by_title(&pool, "Game 1".into()).await.unwrap();
    let media2 = find_media_by_title(&pool, "Game 2".into()).await.unwrap();
    let media3 = find_media_by_title(&pool, "Game 3".into()).await.unwrap();

    assert!(media1.is_some());
    assert!(media2.is_some());
    assert!(media3.is_some());
}

struct FakeMetadataFetcher;

impl MetadataFetcher for FakeMetadataFetcher {
    fn id(&self) -> MetadataProviderId {
        MetadataProviderId::TheInternetGameDatabase
    }
    fn name(&self) -> &'static str {
        "FakeIGDB"
    }
    fn supports_media_type(&self, media_type: MediaType) -> bool {
        media_type == MediaType::Game
    }
}

#[async_trait]
impl GeneralInfoFetcher for FakeMetadataFetcher {
    async fn fetch_general_info(
        &self,
        query: &MetadataQuery<'_>,
    ) -> Result<Option<GeneralMetadata>, crate::metadata::error::MetadataError> {
        if query.title == "Hollow Knight" {
            Ok(Some(GeneralMetadata::Game(
                crate::metadata::models::GameGeneralMetadata {
                    common: crate::metadata::models::CommonMetadata {
                        description: Some("Epic storyline".into()),
                        summary: Some("A bug adventure".into()),
                        release_date: Some(1487894400),
                        genres: vec!["Metroidvania".into(), "Platformer".into()],
                        tags: vec!["Difficult".into(), "2D".into()],
                    },
                    developers: vec!["Team Cherry".into()],
                    publishers: vec!["Team Cherry".into()],
                    platforms: vec!["PC".into()],
                    series: Some("Hollow Knight Series".into()),
                },
            )))
        } else {
            Ok(None)
        }
    }
}

#[tokio::test]
async fn test_sync_with_metadata_step() {
    let pool = setup_db().await;

    let storefronts =
        test_storefront_registry(Arc::new(FakeSteamStorefront::new(vec![discovered_game(
            "Hollow Knight",
        )])));
    let events = EventBus::new();
    let assets = AssetRegistry::new();
    let mut metadata = MetadataRegistry::new();
    metadata.register(Arc::new(FakeMetadataFetcher));

    let service = LibrarySyncService::new(
        uuid::Uuid::new_v4(),
        pool.clone(),
        events,
        crate::agents::AgentManager::new(),
        storefronts,
        assets,
        metadata,
    );

    service.sync_all(false).await.unwrap();

    let media = find_media_by_title(&pool, "Hollow Knight".into())
        .await
        .unwrap()
        .expect("media should exist");

    assert_eq!(media.summary.as_deref(), Some("A bug adventure"));
    assert_eq!(media.description.as_deref(), Some("Epic storyline"));
    assert_eq!(media.release_date, Some(1487894400));

    let full_media = crate::media::service::get_media_by_id(&pool, media.id)
        .await
        .unwrap();
    assert_eq!(full_media.genres.len(), 2);
    assert!(full_media.genres.contains(&"Metroidvania".to_string()));
    assert!(full_media.genres.contains(&"Platformer".to_string()));
    assert_eq!(full_media.tags.len(), 2);
    assert!(full_media.tags.contains(&"Difficult".to_string()));
    assert!(full_media.tags.contains(&"2D".to_string()));

    if let Some(crate::media::models::MediaDetails::Game(details)) = full_media.details {
        assert_eq!(details.series.as_deref(), Some("Hollow Knight Series"));
        assert_eq!(details.developers, vec!["Team Cherry"]);
        assert_eq!(details.publishers, vec!["Team Cherry"]);
    } else {
        panic!("Expected Game details");
    }
}

#[tokio::test]
async fn test_metadata_step_runs_only_once_across_multiple_items() {
    let pool = setup_db().await;

    let storefronts = test_storefront_registry(Arc::new(FakeSteamStorefront::new(vec![
        discovered_game("Hollow Knight"),
        discovered_game("Another Game"),
    ])));
    let events = EventBus::new();
    let assets = AssetRegistry::new();
    let mut metadata = MetadataRegistry::new();
    metadata.register(Arc::new(FakeMetadataFetcher));

    let service = LibrarySyncService::new(
        uuid::Uuid::new_v4(),
        pool.clone(),
        events,
        crate::agents::AgentManager::new(),
        storefronts,
        assets,
        metadata,
    );

    service.sync_all(false).await.unwrap();

    let media1 = find_media_by_title(&pool, "Hollow Knight".into())
        .await
        .unwrap()
        .expect("media 1 should exist");
    assert_eq!(media1.summary.as_deref(), Some("A bug adventure"));

    let media2 = find_media_by_title(&pool, "Another Game".into())
        .await
        .unwrap()
        .expect("media 2 should exist");
    assert_eq!(media2.summary, None);

    let search1 = itonda_database::media::find_metadata_search_by_media_id(&pool, &media1.id)
        .await
        .unwrap();
    assert!(search1.is_some());

    let search2 = itonda_database::media::find_metadata_search_by_media_id(&pool, &media2.id)
        .await
        .unwrap();
    assert!(search2.is_some());
}

struct PartialMetadataFetcher1;
impl MetadataFetcher for PartialMetadataFetcher1 {
    fn id(&self) -> MetadataProviderId {
        MetadataProviderId::TheInternetGameDatabase
    }
    fn name(&self) -> &'static str {
        "Partial1"
    }
    fn supports_media_type(&self, media_type: MediaType) -> bool {
        media_type == MediaType::Game
    }
}
#[async_trait]
impl GeneralInfoFetcher for PartialMetadataFetcher1 {
    async fn fetch_general_info(
        &self,
        _query: &MetadataQuery<'_>,
    ) -> Result<Option<GeneralMetadata>, crate::metadata::error::MetadataError> {
        Ok(Some(GeneralMetadata::Game(
            crate::metadata::models::GameGeneralMetadata {
                common: crate::metadata::models::CommonMetadata {
                    description: None,
                    summary: Some("Summary from Provider 1".into()),
                    release_date: Some(1500000000),
                    genres: vec!["Action".into()],
                    tags: vec!["Hard".into()],
                },
                developers: vec!["Dev A".into()],
                publishers: vec![],
                platforms: vec!["PC".into()],
                series: None,
            },
        )))
    }
}

struct PartialMetadataFetcher2;
impl MetadataFetcher for PartialMetadataFetcher2 {
    fn id(&self) -> MetadataProviderId {
        MetadataProviderId::TheInternetGameDatabase
    }
    fn name(&self) -> &'static str {
        "Partial2"
    }
    fn supports_media_type(&self, media_type: MediaType) -> bool {
        media_type == MediaType::Game
    }
}
#[async_trait]
impl GeneralInfoFetcher for PartialMetadataFetcher2 {
    async fn fetch_general_info(
        &self,
        _query: &MetadataQuery<'_>,
    ) -> Result<Option<GeneralMetadata>, crate::metadata::error::MetadataError> {
        Ok(Some(GeneralMetadata::Game(
            crate::metadata::models::GameGeneralMetadata {
                common: crate::metadata::models::CommonMetadata {
                    description: Some("Description from Provider 2".into()),
                    summary: Some("Summary from Provider 2".into()),
                    release_date: None,
                    genres: vec!["Adventure".into()],
                    tags: vec!["2D".into()],
                },
                developers: vec!["Dev B".into()],
                publishers: vec!["Pub B".into()],
                platforms: vec!["Switch".into()],
                series: Some("Awesome Series".into()),
            },
        )))
    }
}

#[tokio::test]
async fn test_metadata_step_multi_provider_merge() {
    let pool = setup_db().await;

    let storefronts =
        test_storefront_registry(Arc::new(FakeSteamStorefront::new(vec![discovered_game(
            "Multi Game",
        )])));
    let events = EventBus::new();
    let assets = AssetRegistry::new();
    let mut metadata = MetadataRegistry::new();
    metadata.register(Arc::new(PartialMetadataFetcher1));
    metadata.register(Arc::new(PartialMetadataFetcher2));

    let service = LibrarySyncService::new(
        uuid::Uuid::new_v4(),
        pool.clone(),
        events,
        crate::agents::AgentManager::new(),
        storefronts,
        assets,
        metadata,
    );

    service.sync_all(false).await.unwrap();

    let media = find_media_by_title(&pool, "Multi Game".into())
        .await
        .unwrap()
        .expect("media should exist");

    assert_eq!(media.summary.as_deref(), Some("Summary from Provider 1"));
    assert_eq!(
        media.description.as_deref(),
        Some("Description from Provider 2")
    );
    assert_eq!(media.release_date, Some(1500000000));

    let full_media = crate::media::service::get_media_by_id(&pool, media.id)
        .await
        .unwrap();
    assert_eq!(full_media.genres.len(), 2);
    assert!(full_media.genres.contains(&"Action".to_string()));
    assert!(full_media.genres.contains(&"Adventure".to_string()));

    assert_eq!(full_media.tags.len(), 2);
    assert!(full_media.tags.contains(&"Hard".to_string()));
    assert!(full_media.tags.contains(&"2D".to_string()));

    if let Some(crate::media::models::MediaDetails::Game(details)) = full_media.details {
        assert_eq!(details.series.as_deref(), Some("Awesome Series"));
        assert_eq!(details.developers.len(), 2);
        assert!(details.developers.contains(&"Dev A".to_string()));
        assert!(details.developers.contains(&"Dev B".to_string()));
        assert_eq!(details.publishers, vec!["Pub B"]);
    } else {
        panic!("Expected Game details");
    }
}
