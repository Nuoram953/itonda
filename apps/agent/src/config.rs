use itonda_domain::store::{Store, toml::TomlCodec};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::identity::AgentIdentity;

pub const DEFAULT_SERVER_URL: &str = "ws://localhost:3005/ws/agent/connect";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ServerConfig {
    #[serde(default = "default_server_url")]
    pub url: String,
}

fn default_server_url() -> String {
    DEFAULT_SERVER_URL.to_string()
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            url: default_server_url(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AgentConfig {
    pub identity: AgentIdentity,
    #[serde(default)]
    pub server: ServerConfig,
}

impl AgentConfig {
    pub fn server_url(&self) -> String {
        std::env::var("ITONDA_SERVER_URL").unwrap_or_else(|_| self.server.url.clone())
    }
}

impl Default for AgentConfig {
    fn default() -> Self {
        Self {
            identity: AgentIdentity {
                id: Uuid::new_v4().to_string(),
                name: "Itonda Agent".into(),
            },
            server: ServerConfig::default(),
        }
    }
}

pub type AgentConfigStore = Store<AgentConfig, TomlCodec>;
