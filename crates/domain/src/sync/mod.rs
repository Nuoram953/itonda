use crate::storefronts::{models::StorefrontId, registry::StorefrontRegistry};

pub mod context;
pub mod errors;
pub mod events;
pub mod pipeline;

pub struct LibrarySyncService {
    storefronts: StorefrontRegistry,
}

impl LibrarySyncService {
    pub fn new(storefronts: StorefrontRegistry) -> Self {
        Self { storefronts }
    }

    pub async fn sync_storefront(&self, _storefront: StorefrontId) {}

    pub async fn sync_all(&self) {}
}
