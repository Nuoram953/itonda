use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum ServerToAgentMessage {
    Ping,
    Launch(LaunchCommand),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaunchCommand {
    pub request_id: Uuid,
    pub media_id: String,
    pub program: String,
    pub args: Vec<String>,
}
