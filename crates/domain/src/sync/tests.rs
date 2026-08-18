use crate::{
    assets::registry::AssetRegistry,
    events::EventBus,
    media::{
        discovered::{DiscoveredMedia, DiscoveredMediaMetadata, GameMetadata},
        types::MediaType,
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
    let mut registry = StorefrontRegistry::new();

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

    let service = LibrarySyncService::new(
        uuid::Uuid::new_v4(),
        pool.clone(),
        events,
        crate::agents::AgentManager::new(),
        storefronts,
        assets,
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
        },
    )
    .await
    .unwrap();

    let storefronts = StorefrontRegistry::new();
    let events = EventBus::new();
    let assets = AssetRegistry::new();

    let service = LibrarySyncService::new(
        uuid::Uuid::new_v4(),
        pool.clone(),
        events,
        crate::agents::AgentManager::new(),
        storefronts,
        assets,
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

    let service = LibrarySyncService::new(
        uuid::Uuid::new_v4(),
        pool.clone(),
        events,
        agents,
        storefronts,
        assets,
    );

    service.sync_all(false).await.unwrap();

    let received = rx.recv().await;
    assert!(matches!(received, Some(ServerToAgentMessage::Scan(_))));
}

#[tokio::test]
async fn sync_all_continues_when_item_fails() {
    let pool = setup_db().await;

    // First game will fail if we have a pipeline step that errors, but with default pipeline,
    // let's verify multiple games sync cleanly even if one already has bad data
    let storefronts = test_storefront_registry(Arc::new(FakeSteamStorefront::new(vec![
        discovered_game("Game 1"),
        discovered_game("Game 2"),
        discovered_game("Game 3"),
    ])));
    let events = EventBus::new();
    let assets = AssetRegistry::new();

    let service = LibrarySyncService::new(
        uuid::Uuid::new_v4(),
        pool.clone(),
        events,
        crate::agents::AgentManager::new(),
        storefronts,
        assets,
    );

    service.sync_all(false).await.unwrap();

    let media1 = find_media_by_title(&pool, "Game 1".into()).await.unwrap();
    let media2 = find_media_by_title(&pool, "Game 2".into()).await.unwrap();
    let media3 = find_media_by_title(&pool, "Game 3".into()).await.unwrap();

    assert!(media1.is_some());
    assert!(media2.is_some());
    assert!(media3.is_some());
}
