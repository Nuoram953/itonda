use itonda_domain::store::{Store, toml::TomlCodec};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Secrets {
    pub storefronts: StorefrontsSettings,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct StorefrontsSettings {
    pub steam: SteamSettings,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct SteamSettings {
    pub api_key: String,
    pub steam_id: u64,
}

pub type SecretsManager = Store<Secrets, TomlCodec>;
