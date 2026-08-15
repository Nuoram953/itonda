use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::scanner::models::ScannedMedia;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum AgentToServerMessage {
    Pong,
    Register(AgentRegistration),
    ScanResult(ScanResult),
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
