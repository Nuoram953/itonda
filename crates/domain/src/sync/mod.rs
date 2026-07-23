use crate::storefront::models::StorefrontId;

pub mod context;
pub mod errors;
pub mod events;
pub mod pipeline;

pub struct LibrarySyncService {
    storefronts: StorefrontRegistry,
    pipeline: MediaSyncPipeline,
}

impl LibrarySyncService {
    pub async fn sync_storefront(&self, _storefront: StorefrontId) {}

    pub async fn sync_all(&self) {}
}
