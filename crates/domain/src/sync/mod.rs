use sqlx::SqlitePool;
use tracing::{debug, info};
use uuid::Uuid;

use crate::{
    assets::{downloader::AssetDownloader, registry::AssetRegistry},
    events::{EventBus, JobEventType, JobType, SyncEvent},
    storage::path::AppPaths,
    storefronts::{models::StorefrontId, registry::StorefrontRegistry},
    sync::{
        context::{SyncAction, SyncContext},
        errors::SyncError,
        pipeline::{MediaSyncPipeline, SyncStep},
        steps::{assets::AssetStep, identify::IdentifyStep, persist::PersistStep},
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
    db: SqlitePool,
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
        assets: AssetRegistry,
    ) -> Self {
        let steps: Vec<Box<dyn SyncStep>> = vec![
            Box::new(IdentifyStep::new()),
            Box::new(PersistStep::new(db.clone())),
            Box::new(AssetStep::new(
                db.clone(),
                assets,
                AssetDownloader::new(AppPaths::new()),
            )),
        ];
        let pipeline = MediaSyncPipeline::new(steps);
        Self {
            job_id,
            db,
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
        let mut synced_ids = std::collections::HashSet::new();

        for (_, storefront) in self.storefronts.get_all() {
            let discovered_media = storefront.owned_games().await?;

            info!(
                "Found {} items for storefront {}",
                discovered_media.len(),
                storefront.name()
            );
            for media in discovered_media {
                debug!("Syncing {}", media.title);
                let mut context = SyncContext::new(media);

                self.pipeline.execute(&mut context).await?;

                if let Some(media) = &context.media {
                    synced_ids.insert(media.id.clone());

                    if context.action != SyncAction::Unchanged {
                        self.events.publish_job(
                            self.job_id,
                            JobType::Sync,
                            JobEventType::Sync(SyncEvent::MediaSynced {
                                media_id: media.title.clone(),
                            }),
                        );
                    }
                }
            }
        }

        let db_rows = itonda_database::media::find_all(&self.db, None).await?;
        info!("Found {} media items in database", db_rows.len());

        for row in db_rows {
            if synced_ids.contains(&row.id) {
                continue;
            }

            let media = crate::media::models::Media::try_from(row).unwrap();
            debug!("Syncing database media item {}", media.title);
            let mut context = SyncContext::from_media(media.clone());

            info!("{:?}", context);

            self.pipeline.execute(&mut context).await?;

            if context.action != SyncAction::Unchanged {
                self.events.publish_job(
                    self.job_id,
                    JobType::Sync,
                    JobEventType::Sync(SyncEvent::MediaSynced { media_id: media.id }),
                );
            }
        }

        info!("Sync completed");

        Ok(())
    }
}
