use sqlx::SqlitePool;
use tracing::info;
use uuid::Uuid;

use crate::{
    events::{EventBus, JobEventType, JobType, SyncEvent},
    storefronts::{models::StorefrontId, registry::StorefrontRegistry},
    sync::{
        context::SyncContext,
        errors::SyncError,
        pipeline::{MediaSyncPipeline, SyncStep},
        steps::{identify::IdentifyStep, persist::PersistStep},
    },
};

pub mod context;
pub mod errors;
pub mod events;
pub mod pipeline;
pub mod steps;

#[cfg(test)]
pub mod tests;

pub struct LibrarySyncService {
    job_id: Uuid,
    storefronts: StorefrontRegistry,
    events: EventBus,
    pipeline: MediaSyncPipeline,
}

impl LibrarySyncService {
    pub fn new(
        job_id: Uuid,
        db: SqlitePool,
        events: EventBus,
        storefronts: StorefrontRegistry,
    ) -> Self {
        let steps: Vec<Box<dyn SyncStep>> = vec![
            Box::new(IdentifyStep::new()),
            Box::new(PersistStep::new(db)),
        ];
        let pipeline = MediaSyncPipeline::new(steps);
        Self {
            job_id,
            storefronts,
            events,
            pipeline,
        }
    }

    pub async fn sync_storefront(&self, _storefront: StorefrontId) -> Result<(), SyncError> {
        Ok(())
    }

    pub async fn sync_all(&self) -> Result<(), SyncError> {
        info!("Starting sync process for all");

        for (_, storefront) in self.storefronts.get_all() {
            info!("Syncing from storefront: {}", storefront.name());
            let discovered_media = storefront.owned_games().await?;

            for media in discovered_media {
                info!("Syncing {}", media.title);
                let mut context = SyncContext::new(media);

                self.pipeline.execute(&mut context).await?;

                self.events.publish_job(
                    self.job_id,
                    JobType::Sync,
                    JobEventType::Sync(SyncEvent::MediaSynced {
                        media_id: context.media.unwrap().id,
                    }),
                )
            }
        }

        Ok(())
    }
}
