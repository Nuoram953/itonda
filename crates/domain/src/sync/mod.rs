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
        context::SyncContext,
        errors::SyncError,
        pipeline::{MediaSyncPipeline, SyncStep},
        steps::{
            assets::AssetStep, download::DownloadStep, identify::IdentifyStep,
            metadata::MetadataStep, persist::PersistStep,
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
            Box::new(DownloadStep::new(db.clone(), AppPaths::new())),
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

    pub async fn discover_storefronts(
        &self,
        storefront_filter: Option<StorefrontId>,
    ) -> Result<Vec<String>, SyncError> {
        let mut discovered_ids = Vec::new();

        for (id, storefront) in self.storefronts.get_all() {
            if storefront_filter.is_some_and(|filter| id != filter) {
                continue;
            }

            self.events.publish_job(
                self.job_id,
                JobType::Sync,
                JobEventType::Sync(SyncEvent::StepProgress {
                    media_id: None,
                    title: None,
                    step: "discovery".to_string(),
                    message: format!("Scanning {} for games...", storefront.name()),
                }),
            );

            let discovered_media = storefront.owned_games().await?;

            info!(
                "Found {} items for storefront {}",
                discovered_media.len(),
                storefront.name()
            );

            self.events.publish_job(
                self.job_id,
                JobType::Sync,
                JobEventType::Sync(SyncEvent::StepProgress {
                    media_id: None,
                    title: None,
                    step: "discovery".to_string(),
                    message: format!(
                        "Found {} games in {}",
                        discovered_media.len(),
                        storefront.name()
                    ),
                }),
            );

            let persist = PersistStep::new(self.db.clone());
            for media in discovered_media {
                debug!("Persisting discovered media {}", media.title);
                let mut context = SyncContext::new(media, false);

                if let Err(err) = persist.execute(&mut context).await {
                    tracing::warn!("Failed to persist discovered media: {err}");
                    continue;
                }

                if let Some(persisted) = context.media {
                    discovered_ids.push(persisted.id);
                }
            }
        }

        Ok(discovered_ids)
    }

    pub async fn sync_media(&self, media_id: &str, force: bool) -> Result<(), SyncError> {
        info!("Starting sync process for media {}", media_id);

        let media = crate::media::service::get_media_by_id(&self.db, media_id.to_string()).await?;
        debug!("Syncing database media item {}", media.title);

        let mut context = SyncContext::from_media(media.clone());
        context.force = force;

        let events = self.events.clone();
        let job_id = self.job_id;
        let title = media.title.clone();
        let m_id = media.id.clone();

        let res = self
            .pipeline
            .execute_with_listener(&mut context, |step_name| {
                let msg = match step_name {
                    "Metadata" => Some("Fetching metadata..."),
                    "Assets" => Some("Searching artwork..."),
                    "Download" => Some("Downloading media..."),
                    _ => None,
                };

                if let Some(message) = msg {
                    events.publish_job(
                        job_id,
                        JobType::Sync,
                        JobEventType::Sync(SyncEvent::StepProgress {
                            media_id: Some(m_id.clone()),
                            title: Some(title.clone()),
                            step: step_name.to_lowercase(),
                            message: message.to_string(),
                        }),
                    );
                }
            })
            .await;

        if let Err(err) = res {
            tracing::warn!("Failed to sync database media '{}': {err}", media.title);
            self.events.publish_job(
                self.job_id,
                JobType::Sync,
                JobEventType::Sync(SyncEvent::MediaSyncFailed {
                    media_id: media.id,
                    error: err.to_string(),
                }),
            );
            return Err(err);
        }

        self.events.publish_job(
            self.job_id,
            JobType::Sync,
            JobEventType::Sync(SyncEvent::MediaSynced { media_id: media.id }),
        );

        info!("Sync completed for media {}", media_id);

        Ok(())
    }

    pub async fn sync_storefront(
        &self,
        storefront: StorefrontId,
        force: bool,
    ) -> Result<(), SyncError> {
        self.sync_filtered(force, Some(storefront)).await
    }

    pub async fn sync_all(&self, force: bool) -> Result<(), SyncError> {
        self.sync_filtered(force, None).await
    }

    pub async fn sync_filtered(
        &self,
        force: bool,
        storefront_filter: Option<StorefrontId>,
    ) -> Result<(), SyncError> {
        info!(
            "Starting sync process (storefront filter: {:?})",
            storefront_filter
        );

        let discovered_ids = self.discover_storefronts(storefront_filter).await?;

        let target_ids: Vec<String> = if storefront_filter.is_some() {
            discovered_ids
        } else {
            let db_rows = itonda_database::media::find_all(&self.db, None).await?;
            db_rows.into_iter().map(|row| row.id).collect()
        };

        let total = target_ids.len();
        info!("Found {} media items to sync", total);

        for (idx, media_id) in target_ids.into_iter().enumerate() {
            if total > 1 {
                self.events.publish_job(
                    self.job_id,
                    JobType::Sync,
                    JobEventType::Sync(SyncEvent::StepProgress {
                        media_id: Some(media_id.clone()),
                        title: None,
                        step: "syncing".to_string(),
                        message: format!("Syncing item {} of {}...", idx + 1, total),
                    }),
                );
            }

            if let Err(err) = self.sync_media(&media_id, force).await {
                tracing::warn!("Failed to sync media '{media_id}': {err}");
            }
        }

        if storefront_filter.is_none() {
            self.agents.scan_all().await?;
        }

        info!("Sync completed");

        Ok(())
    }
}
