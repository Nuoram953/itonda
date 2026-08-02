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
    Database(#[from] itonda_database::error::DatabaseError),

    #[error("database error: {0}")]
    Sqlx(sqlx::Error),

    #[error("invalid asset type")]
    InvalidAssetType,

    #[error("invalid media status")]
    InvalidMediaStatus,
}

impl From<sqlx::Error> for MediaError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => MediaError::NotFound,
            err => MediaError::Sqlx(err),
        }
    }
}
