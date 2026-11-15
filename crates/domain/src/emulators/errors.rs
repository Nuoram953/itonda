use itonda_database::error::DatabaseError;

#[derive(Debug, thiserror::Error)]
pub enum EmulatorError {
    #[error("emulator not found")]
    NotFound,

    #[error("emulator with this name already exists")]
    AlreadyExists,

    #[error("database error: {0}")]
    Database(DatabaseError),
}

impl From<DatabaseError> for EmulatorError {
    fn from(err: DatabaseError) -> Self {
        match err {
            DatabaseError::NotFound => Self::NotFound,
            DatabaseError::Sqlx(sqlx_err)
                if sqlx_err
                    .as_database_error()
                    .and_then(|e| e.code())
                    .as_deref()
                    == Some("2067") =>
            {
                Self::AlreadyExists
            }
            err => Self::Database(err),
        }
    }
}
