use crate::{
    storefronts::{models::StorefrontId, registry::StorefrontRegistry},
    sync::{
        context::SyncContext,
        errors::SyncError,
        pipeline::{MediaSyncPipeline, SyncStep},
        steps::identify::IdentifyStep,
    },
};

pub mod context;
pub mod errors;
pub mod events;
pub mod pipeline;
pub mod steps;

pub struct LibrarySyncService {
    storefronts: StorefrontRegistry,
    pipeline: MediaSyncPipeline,
}

impl LibrarySyncService {
    pub fn new(storefronts: StorefrontRegistry) -> Self {
        let steps: Vec<Box<dyn SyncStep>> = vec![Box::new(IdentifyStep::new())];
        let pipeline = MediaSyncPipeline::new(steps);
        Self {
            storefronts,
            pipeline,
        }
    }

    pub async fn sync_storefront(&self, _storefront: StorefrontId) -> Result<(), SyncError> {
        Ok(())
    }

    pub async fn sync_all(&self) -> Result<(), SyncError> {
        for (_, storefront) in self.storefronts.get_all() {
            let discovered_media = storefront.owned_games().await?;

            for media in discovered_media {
                let mut context = SyncContext::new(media);

                self.pipeline.execute(&mut context).await?;
            }
        }

        Ok(())
    }
}
