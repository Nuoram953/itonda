use itonda_domain::store::{Store, toml::TomlCodec};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::identity::AgentIdentity;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    pub identity: AgentIdentity,
}

impl Default for AgentConfig {
    fn default() -> Self {
        Self {
            identity: AgentIdentity {
                id: Uuid::new_v4().to_string(),
                name: "Itonda Agent".into(),
            },
        }
    }
}

pub type AgentConfigStore = Store<AgentConfig, TomlCodec>;
