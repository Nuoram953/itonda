use thiserror::Error;

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("database error: {0}")]
    Sqlx(sqlx::Error),

    #[error("not found")]
    NotFound,

    #[error("migration failed")]
    MigrationFailed,
}

impl From<sqlx::Error> for DatabaseError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => DatabaseError::NotFound,
            err => DatabaseError::Sqlx(err),
        }
    }
}
