use serde::{Deserialize, Serialize};

use crate::config::store::TomlStore;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct AppConfig {
    pub server: ServerConfig,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            server: ServerConfig {
                host: String::from("0.0.0.0"),
                port: 3005,
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: usize,
}

pub type AppConfigManager = TomlStore<AppConfig>;
