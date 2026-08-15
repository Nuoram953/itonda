use itonda_database::error::DatabaseError;

#[derive(Debug, thiserror::Error)]
pub enum AgentsError {
    #[error("database error: {0}")]
    Database(DatabaseError),

    #[error("agent not connected: {0}")]
    NotConnected(String),

    #[error("failed to send message to agent: {0}")]
    SendFailed(String),
}

impl From<DatabaseError> for AgentsError {
    fn from(err: DatabaseError) -> Self {
        AgentsError::Database(err)
    }
}
