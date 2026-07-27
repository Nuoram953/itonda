use crate::{
    events::EventBus,
    media::models::{DiscoveredMedia, DiscoveredMediaMetadata, MediaType},
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
        external_id: "123".to_string(),
        metadata: DiscoveredMediaMetadata {
            total_playtime: None,
        },
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

    let service = LibrarySyncService::new(uuid::Uuid::new_v4(), pool.clone(), events, storefronts);

    service.sync_all().await.unwrap();

    let media = find_media_by_title(&pool, "Portal 2".into()).await.unwrap();

    assert!(media.is_some());
}
