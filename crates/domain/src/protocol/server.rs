use serde::{Deserialize, Serialize};

// Server -> agent

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum ServerMessage {
    Ping,
}
