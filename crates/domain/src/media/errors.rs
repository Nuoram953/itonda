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
}
