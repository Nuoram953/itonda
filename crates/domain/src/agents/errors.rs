use itonda_database::error::DatabaseError;

#[derive(Debug, thiserror::Error)]
pub enum AgentsError {
    #[error("database error: {0}")]
    Database(DatabaseError),
}

impl From<DatabaseError> for AgentsError {
    fn from(err: DatabaseError) -> Self {
        match err {
            DatabaseError::NotFound => AgentsError::Database(err),
            err => AgentsError::Database(err),
        }
    }
}
