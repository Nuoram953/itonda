use serde::{Deserialize, Serialize};

// Agent -> Server

#[derive(Debug, Serialize, Deserialize)]
pub struct AgentRegistration {
    pub id: String,
    pub name: String,
    pub hostname: String,
    pub platform: String,
    pub agent_version: String,
    pub ip_address: String,
}
