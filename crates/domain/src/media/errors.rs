use crate::assets::error::AssetError;
use itonda_database::error::DatabaseError;

#[derive(Debug, thiserror::Error)]
pub enum MediaError {
    #[error("title cannot be empty")]
    InvalidTitle,

    #[error("media not found")]
    NotFound,

    #[error("invalid media type")]
    InvalidMediaType,

    #[error("invalid media id")]
    InvalidId,

    #[error("database error: {0}")]
    Database(DatabaseError),

    #[error("invalid asset type")]
    InvalidAssetType,

    #[error("invalid media status")]
    InvalidMediaStatus,

    #[error("asset error: {0}")]
    AssetError(#[from] AssetError),

    #[error("storefront error: {0}")]
    StorefrontError(#[from] crate::storefronts::error::StorefrontError),
}

impl From<DatabaseError> for MediaError {
    fn from(err: DatabaseError) -> Self {
        match err {
            DatabaseError::NotFound => MediaError::NotFound,
            err => MediaError::Database(err),
        }
    }
}
