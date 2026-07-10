use thiserror::Error;

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("database error: {0}")]
    Sqlx(#[from] sqlx::Error),

    #[error("media not found")]
    MediaNotFound,

    #[error("migration failed")]
    MigrationFailed,
}
