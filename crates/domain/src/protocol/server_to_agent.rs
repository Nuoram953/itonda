use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::media::types::MediaType;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum ServerToAgentMessage {
    Ping,
    Launch(LaunchCommand),
    Scan(ScanCommand),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaunchCommand {
    pub request_id: Uuid,
    pub media_id: String,
    pub launch_id: String,
    pub program: String,
    pub args: Vec<String>,
    pub working_directory: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanCommand {
    pub request_id: Uuid,
    pub media_type: Option<MediaType>,
    pub source: Option<String>,
}
