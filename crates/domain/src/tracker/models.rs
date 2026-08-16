use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TrackTarget {
    pub media_id: String,
    pub working_directory: Option<PathBuf>,
    pub process_name: Option<String>,
    pub program: Option<String>,
}

impl TrackTarget {
    pub fn from_directory(media_id: impl Into<String>, dir: impl Into<PathBuf>) -> Self {
        Self {
            media_id: media_id.into(),
            working_directory: Some(dir.into()),
            process_name: None,
            program: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub exe: Option<PathBuf>,
    pub cwd: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TrackingSession {
    pub media_id: String,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub stopped_at: Option<chrono::DateTime<chrono::Utc>>,
    pub duration_seconds: u64,
}
