use serde::{Deserialize, Serialize};

use crate::media::types::{MediaLaunchType, MediaType};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ScannedMedia {
    pub media_type: MediaType,
    pub title: String,
    pub external_id: Option<String>,
    pub source: String,
    pub working_directory: Option<String>,
    pub launch: Option<ScannedLaunch>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ScannedLaunch {
    pub name: String,
    pub launch_type: MediaLaunchType,
    pub program: String,
    pub arguments: Vec<String>,
    pub working_directory: Option<String>,
}
