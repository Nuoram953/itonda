use itonda_database::error::DatabaseError;
use thiserror::Error;

use crate::{
    agents::errors::AgentsError, assets::error::AssetError, media::errors::MediaError,
    metadata::error::MetadataError, storefronts::error::StorefrontError,
};

#[derive(Debug, Error)]
pub enum SyncError {
    #[error("database error: {0}")]
    Database(#[from] DatabaseError),

    #[error("media error: {0}")]
    Media(#[from] MediaError),

    #[error("storefront error: {0}")]
    Storefront(#[from] StorefrontError),

    #[error("Asset error: {0}")]
    Asset(#[from] AssetError),

    #[error("metadata error: {0}")]
    Metadata(#[from] MetadataError),

    #[error("agent error: {0}")]
    Agent(#[from] AgentsError),

    #[error("missing media in sync context")]
    MissingMedia,

    #[error("missing discovered media")]
    MissingDiscoveredMedia,

    #[error("serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("sync step failed: {step}: {source}")]
    Step {
        step: String,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },
}
