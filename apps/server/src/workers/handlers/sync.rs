use itonda_domain::{
    agents::AgentManager,
    assets::registry::AssetRegistry,
    events::{EventBus, JobEventType, JobType, SyncEvent},
    metadata::registry::MetadataRegistry,
    storefronts::registry::StorefrontRegistry,
    sync::LibrarySyncService,
};
use sqlx::SqlitePool;

use crate::workers::jobs::SyncJob;

pub struct SyncHandler {
    db: SqlitePool,
    events: EventBus,
    agents: AgentManager,
    storefronts: StorefrontRegistry,
    assets: AssetRegistry,
    metadata: MetadataRegistry,
}

impl SyncHandler {
    pub fn new(
        db: SqlitePool,
        events: EventBus,
        agents: AgentManager,
        storefronts: StorefrontRegistry,
        assets: AssetRegistry,
        metadata: MetadataRegistry,
    ) -> Self {
        Self {
            db,
            events,
            agents,
            storefronts,
            assets,
            metadata,
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
            self.agents.clone(),
            self.storefronts.clone(),
            self.assets.clone(),
            self.metadata.clone(),
        );

        let _ = sync.sync_all(job.force).await;

        self.events.publish_job(
            job.id,
            JobType::Sync,
            JobEventType::Sync(SyncEvent::Completed),
        );
    }
}
