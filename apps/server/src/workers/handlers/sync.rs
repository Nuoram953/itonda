use crate::{events::EventBus, state::AppState, workers::jobs::SyncJob};

use itonda_domain::{storefronts::registry::StorefrontRegistry, sync::LibrarySyncService};
use sqlx::SqlitePool;

pub struct SyncHandler {
    db: SqlitePool,
    events: EventBus,
    storefronts: StorefrontRegistry,
}

impl SyncHandler {
    pub fn new(db: SqlitePool, events: EventBus, storefronts: StorefrontRegistry) -> Self {
        Self {
            db,
            events,
            storefronts,
        }
    }

    pub async fn handle(&self, _job: SyncJob) {
        let sync = LibrarySyncService::new(self.storefronts.clone());
        sync.sync_all().await;
    }
}
