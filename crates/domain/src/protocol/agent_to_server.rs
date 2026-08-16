use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::scanner::models::ScannedMedia;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum AgentToServerMessage {
    Pong,
    Register(AgentRegistration),
    ScanResult(ScanResult),
    MediaStarted(MediaStartedPayload),
    MediaStopped(MediaStoppedPayload),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentRegistration {
    pub id: String,
    pub name: String,
    pub hostname: String,
    pub platform: String,
    pub agent_version: String,
    pub ip_address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanResult {
    pub request_id: Uuid,
    pub agent_id: String,
    pub items: Vec<ScannedMedia>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MediaStartedPayload {
    pub media_id: String,
    pub agent_id: String,
    pub launch_id: String,
    pub started_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MediaStoppedPayload {
    pub media_id: String,
    pub agent_id: String,
    pub launch_id: String,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub stopped_at: chrono::DateTime<chrono::Utc>,
    pub duration_seconds: u64,
}
