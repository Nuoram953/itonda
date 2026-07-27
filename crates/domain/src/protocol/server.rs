use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Server -> agent

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum ServerMessage {
    Ping,
    Launch(LaunchCommand),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LaunchCommand {
    pub request_id: Uuid,
    pub media_id: String,
    pub program: String,
    pub args: Vec<String>,
}
