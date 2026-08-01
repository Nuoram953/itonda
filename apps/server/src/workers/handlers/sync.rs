use itonda_domain::{
    assets::registry::AssetRegistry,
    events::{EventBus, JobEventType, JobType, SyncEvent},
    storefronts::registry::StorefrontRegistry,
    sync::LibrarySyncService,
};
use sqlx::SqlitePool;

use crate::workers::jobs::SyncJob;

pub struct SyncHandler {
    db: SqlitePool,
    events: EventBus,
    storefronts: StorefrontRegistry,
    assets: AssetRegistry,
}

impl SyncHandler {
    pub fn new(
        db: SqlitePool,
        events: EventBus,
        storefronts: StorefrontRegistry,
        assets: AssetRegistry,
    ) -> Self {
        Self {
            db,
            events,
            storefronts,
            assets,
        }
    }

    pub async fn handle(&self, job: SyncJob) {
        self.events.publish_job(
            job.id,
            JobType::Sync,
            JobEventType::Sync(SyncEvent::Started),
        );

        let sync = LibrarySyncService::new(
            job.id,
            self.db.clone(),
            self.events.clone(),
            self.storefronts.clone(),
            self.assets.clone(),
        );

        let _ = sync.sync_all().await;

        self.events.publish_job(
            job.id,
            JobType::Sync,
            JobEventType::Sync(SyncEvent::Completed),
        );
    }
}
