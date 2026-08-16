use thiserror::Error;

#[derive(Debug, Error)]
pub enum TrackerError {
    #[error("Target directory not specified or invalid: {0}")]
    InvalidDirectory(String),

    #[error("System process inspection failed: {0}")]
    ProcessInspection(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
