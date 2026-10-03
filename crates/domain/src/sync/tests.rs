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
                        external_ids: vec![],
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
                    external_ids: vec![],
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
                    external_ids: vec![],
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

struct CountingMetadataFetcher {
    provider_id: MetadataProviderId,
    name: &'static str,
    meta: Option<GeneralMetadata>,
    calls: Arc<std::sync::atomic::AtomicUsize>,
}

impl MetadataFetcher for CountingMetadataFetcher {
    fn id(&self) -> MetadataProviderId {
        self.provider_id
    }
    fn name(&self) -> &'static str {
        self.name
    }
    fn supports_media_type(&self, media_type: MediaType) -> bool {
        media_type == MediaType::Game
    }
}

#[async_trait]
impl GeneralInfoFetcher for CountingMetadataFetcher {
    async fn fetch_general_info(
        &self,
        _query: &MetadataQuery<'_>,
    ) -> Result<Option<GeneralMetadata>, crate::metadata::error::MetadataError> {
        self.calls.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        Ok(self.meta.clone())
    }
}

#[tokio::test]
async fn test_metadata_step_skips_when_media_already_complete() {
    use crate::sync::pipeline::SyncStep;
    use std::sync::atomic::{AtomicUsize, Ordering};

    let pool = setup_db().await;
    let calls = Arc::new(AtomicUsize::new(0));

    let media_row = itonda_database::media::insert_media(
        &pool,
        itonda_database::media::MediaInsert {
            title: "Complete Game".into(),
            media_type: "game".into(),
            status_id: 1,
            description: Some("Description".into()),
            summary: Some("Summary".into()),
            release_date: Some(12345),
        },
    )
    .await
    .unwrap();

    let mut media = crate::media::models::Media::try_from(media_row).unwrap();
    media.genres = vec!["Action".into()];
    media.tags = vec!["Singleplayer".into()];
    media.details = Some(crate::media::models::MediaDetails::Game(
        crate::media::models::MediaGameDetails {
            playtime_minutes: None,
            last_played_at: None,
            series: Some("Series".into()),
            developers: vec!["Dev".into()],
            publishers: vec!["Pub".into()],
        },
    ));

    let mut metadata = MetadataRegistry::new();
    metadata.register(Arc::new(CountingMetadataFetcher {
        provider_id: MetadataProviderId::TheInternetGameDatabase,
        name: "CountingFetcher",
        meta: None,
        calls: calls.clone(),
    }));

    let step = crate::sync::steps::metadata::MetadataStep::new(pool.clone(), metadata);
    let mut context = crate::sync::context::SyncContext::from_media(media);

    step.execute(&mut context).await.unwrap();

    // Because the game's metadata is already complete according to policy,
    // the external metadata API should NOT be called at all.
    assert_eq!(calls.load(Ordering::SeqCst), 0);
}

#[tokio::test]
async fn test_metadata_step_skips_already_searched_store_but_calls_new_store() {
    use std::sync::atomic::{AtomicUsize, Ordering};

    let pool = setup_db().await;
    let calls_a = Arc::new(AtomicUsize::new(0));
    let calls_b = Arc::new(AtomicUsize::new(0));

    let storefronts =
        test_storefront_registry(Arc::new(FakeSteamStorefront::new(vec![discovered_game(
            "Partial Game",
        )])));
    let events = EventBus::new();
    let assets = AssetRegistry::new();

    // 1st sync: only Store A registered, finds nothing
    let mut metadata1 = MetadataRegistry::new();
    metadata1.register(Arc::new(CountingMetadataFetcher {
        provider_id: MetadataProviderId::TheInternetGameDatabase,
        name: "StoreA",
        meta: None,
        calls: calls_a.clone(),
    }));

    let service1 = LibrarySyncService::new(
        uuid::Uuid::new_v4(),
        pool.clone(),
        events.clone(),
        crate::agents::AgentManager::new(),
        storefronts.clone(),
        assets.clone(),
        metadata1,
    );

    service1.sync_all(false).await.unwrap();
    assert_eq!(calls_a.load(Ordering::SeqCst), 1);

    let media = find_media_by_title(&pool, "Partial Game".into())
        .await
        .unwrap()
        .expect("media should exist");

    let searches1 = itonda_database::media::find_metadata_searches_by_media_id(&pool, &media.id)
        .await
        .unwrap();
    assert_eq!(searches1.len(), 1);
    assert_eq!(searches1[0].store_id, "igdb");
    assert_eq!(searches1[0].metadata_type, "general");

    // 2nd sync: Store A and newly added Store B registered
    let mut metadata2 = MetadataRegistry::new();
    metadata2.register(Arc::new(CountingMetadataFetcher {
        provider_id: MetadataProviderId::TheInternetGameDatabase,
        name: "StoreA",
        meta: None,
        calls: calls_a.clone(),
    }));
    metadata2.register(Arc::new(CountingMetadataFetcher {
        provider_id: MetadataProviderId::HowLongToBeat,
        name: "StoreB",
        meta: Some(GeneralMetadata::Game(
            crate::metadata::models::GameGeneralMetadata {
                common: crate::metadata::models::CommonMetadata {
                    summary: Some("Found by Store B".into()),
                    ..Default::default()
                },
                ..Default::default()
            },
        )),
        calls: calls_b.clone(),
    }));

    let service2 = LibrarySyncService::new(
        uuid::Uuid::new_v4(),
        pool.clone(),
        events,
        crate::agents::AgentManager::new(),
        storefronts,
        assets,
        metadata2,
    );

    service2.sync_all(false).await.unwrap();

    // Store A was NOT called again (already tried), but Store B was called
    assert_eq!(calls_a.load(Ordering::SeqCst), 1);
    assert_eq!(calls_b.load(Ordering::SeqCst), 1);

    let searches2 = itonda_database::media::find_metadata_searches_by_media_id(&pool, &media.id)
        .await
        .unwrap();
    assert_eq!(searches2.len(), 2);
    let store_ids: Vec<String> = searches2.into_iter().map(|s| s.store_id).collect();
    assert!(store_ids.contains(&"igdb".to_string()));
    assert!(store_ids.contains(&"howlongtobeat".to_string()));
}

#[tokio::test]
async fn test_metadata_step_searches_different_metadata_type_independently() {
    let pool = setup_db().await;

    let media = itonda_database::media::insert_media(
        &pool,
        itonda_database::media::MediaInsert {
            title: "Test Game".into(),
            media_type: "game".into(),
            status_id: 1,
            ..Default::default()
        },
    )
    .await
    .unwrap();

    // Insert general search
    itonda_database::media::insert_media_metadata_search(
        &pool,
        itonda_database::media::MediaMetadataSearchInsert {
            media_id: media.id.clone(),
            store_id: "igdb".into(),
            metadata_type: "general".into(),
        },
    )
    .await
    .unwrap();

    // New metadata type (e.g. HowLongToBeat) can be searched and recorded independently
    itonda_database::media::insert_media_metadata_search(
        &pool,
        itonda_database::media::MediaMetadataSearchInsert {
            media_id: media.id.clone(),
            store_id: "howlongtobeat".into(),
            metadata_type: "how_long_to_beat".into(),
        },
    )
    .await
    .unwrap();

    let all_searches = itonda_database::media::find_metadata_searches_by_media_id(&pool, &media.id)
        .await
        .unwrap();
    assert_eq!(all_searches.len(), 2);

    let general_search = all_searches
        .iter()
        .find(|s| s.metadata_type == "general")
        .unwrap();
    assert_eq!(general_search.store_id, "igdb");
    assert_eq!(
        general_search.idempotency_key.as_deref(),
        Some(format!("{}:igdb:general", media.id).as_str())
    );

    let hltb_search = all_searches
        .iter()
        .find(|s| s.metadata_type == "how_long_to_beat")
        .unwrap();
    assert_eq!(hltb_search.store_id, "howlongtobeat");
    assert_eq!(
        hltb_search.idempotency_key.as_deref(),
        Some(format!("{}:howlongtobeat:how_long_to_beat", media.id).as_str())
    );
}
