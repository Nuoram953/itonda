use itonda_domain::store::{Store, toml::TomlCodec};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(default)]
pub struct AppConfig {
    pub server: ServerConfig,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(default)]
pub struct ServerConfig {
    pub host: String,
    pub port: usize,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: String::from("0.0.0.0"),
            port: 3005,
        }
    }
}

pub type AppConfigManager = Store<AppConfig, TomlCodec>;

impl AppConfig {
    pub fn apply_patch(&mut self, patch: PatchAppConfig) {
        if let Some(server) = patch.server {
            self.server.apply_patch(server);
        }
    }
}

impl ServerConfig {
    pub fn apply_patch(&mut self, patch: PatchServerConfig) {
        if let Some(host) = patch.host {
            self.host = host;
        }
        if let Some(port) = patch.port {
            self.port = port;
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct PatchAppConfig {
    pub server: Option<PatchServerConfig>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct PatchServerConfig {
    pub host: Option<String>,
    pub port: Option<usize>,
}
