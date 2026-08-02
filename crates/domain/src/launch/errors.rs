use itonda_database::error::DatabaseError;

#[derive(Debug, thiserror::Error)]
pub enum LaunchError {
    #[error("launch id not found")]
    NotFound,

    #[error("invalid launch id")]
    InvalidId,

    #[error("no agent available to launch a media")]
    NoAgentAvailable,

    #[error("database error: {0}")]
    Database(DatabaseError),
}

impl From<DatabaseError> for LaunchError {
    fn from(err: DatabaseError) -> Self {
        match err {
            DatabaseError::NotFound => LaunchError::NotFound,
            err => LaunchError::Database(err),
        }
    }
}
