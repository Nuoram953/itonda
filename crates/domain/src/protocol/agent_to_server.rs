use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum AgentToServerMessage {
    Register(AgentRegistration),
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
