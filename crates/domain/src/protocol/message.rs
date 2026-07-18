use serde::{Deserialize, Serialize};

use crate::protocol::agent::AgentRegistration;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum AgentMessage {
    Ping,
    Register(AgentRegistration),
}
