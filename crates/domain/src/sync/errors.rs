use itonda_database::error::DatabaseError;
use thiserror::Error;

use crate::storefronts::error::StorefrontError;

#[derive(Debug, Error)]
pub enum SyncError {
    #[error("database error: {0}")]
    Database(#[from] DatabaseError),

    #[error("storefront error: {0}")]
    Storefront(#[from] StorefrontError),

    #[error("missing media in sync context")]
    MissingMedia,

    #[error("missing discovered media")]
    MissingDiscoveredMedia,

    #[error("sync step failed: {step}: {source}")]
    Step {
        step: String,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },
}
