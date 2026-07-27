use serde::{Deserialize, Serialize};

use crate::protocol::{agent::AgentRegistration, server::LaunchCommand};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum AgentMessage {
    Ping,
    Register(AgentRegistration),
    Launch(LaunchCommand),
}
