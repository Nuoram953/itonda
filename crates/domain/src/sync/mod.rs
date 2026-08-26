use sqlx::SqlitePool;
use tracing::{debug, info};
use uuid::Uuid;

use crate::{
    agents::AgentManager,
    assets::{downloader::AssetDownloader, registry::AssetRegistry},
    events::{EventBus, JobEventType, JobType, SyncEvent},
    metadata::registry::MetadataRegistry,
    storage::path::AppPaths,
    storefronts::{models::StorefrontId, registry::StorefrontRegistry},
    sync::{
        context::{SyncAction, SyncContext},
        errors::SyncError,
        pipeline::{MediaSyncPipeline, SyncStep},
        steps::{
            assets::AssetStep, identify::IdentifyStep, metadata::MetadataStep, persist::PersistStep,
        },
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
    agents: AgentManager,
    storefronts: StorefrontRegistry,
    events: EventBus,
    pipeline: MediaSyncPipeline,
}

impl LibrarySyncService {
    pub fn new(
        job_id: Uuid,
        db: SqlitePool,
        events: EventBus,
        agents: AgentManager,
        storefronts: StorefrontRegistry,
        assets: AssetRegistry,
        metadata: MetadataRegistry,
    ) -> Self {
        let steps: Vec<Box<dyn SyncStep>> = vec![
            Box::new(IdentifyStep::new()),
            Box::new(PersistStep::new(db.clone())),
            Box::new(MetadataStep::new(db.clone(), metadata)),
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
            agents,
            storefronts,
            events,
            pipeline,
        }
    }

    pub async fn sync_storefront(&self, _storefront: StorefrontId) -> Result<(), SyncError> {
        Ok(())
    }

    pub async fn sync_all(&self, force: bool) -> Result<(), SyncError> {
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
                let mut context = SyncContext::new(media, force);

                if let Err(err) = self.pipeline.execute(&mut context).await {
                    tracing::warn!(
                        "Failed to sync media '{}': {err}",
                        context
                            .discovered
                            .as_ref()
                            .map(|d| d.title.as_str())
                            .unwrap_or("unknown")
                    );
                    continue;
                }

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

            if let Err(err) = self.pipeline.execute(&mut context).await {
                tracing::warn!("Failed to sync database media '{}': {err}", media.title);
                continue;
            }

            if context.action != SyncAction::Unchanged {
                self.events.publish_job(
                    self.job_id,
                    JobType::Sync,
                    JobEventType::Sync(SyncEvent::MediaSynced { media_id: media.id }),
                );
            }
        }

        self.agents.scan_all().await?;

        info!("Sync completed");

        Ok(())
    }
}
