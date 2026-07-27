#[derive(Debug, thiserror::Error)]
pub enum LaunchError {
    #[error("launch id not found")]
    NotFound,

    #[error("invalid launch id")]
    InvalidId,

    #[error("no agent available to launch a media")]
    NoAgentAvailable,

    #[error("database error: {0}")]
    Database(#[from] itonda_database::error::DatabaseError),
}
