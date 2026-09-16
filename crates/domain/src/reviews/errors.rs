use itonda_database::error::DatabaseError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ReviewError {
    #[error("Database error: {0}")]
    Database(#[from] DatabaseError),

    #[error("Media not found: {0}")]
    MediaNotFound(String),

    #[error("Thought not found: {0}")]
    ThoughtNotFound(String),

    #[error("Invalid verdict: {0}")]
    InvalidVerdict(String),

    #[error("Validation error: {0}")]
    Validation(String),
}
