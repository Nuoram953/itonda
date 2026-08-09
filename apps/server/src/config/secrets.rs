use itonda_domain::store::{Store, toml::TomlCodec};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Secrets {
    pub storefronts: StorefrontsSettings,
    pub asset_store: AssetStoreSettings,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct StorefrontsSettings {
    pub steam: SteamSettings,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct AssetStoreSettings {
    pub steam_grid_db: SteamGridDbSettings,
    pub tmdb: TheMovieDatabaseSettings,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct SteamSettings {
    pub api_key: String,
    pub steam_id: u64,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct SteamGridDbSettings {
    pub api_key: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct TheMovieDatabaseSettings {
    pub api_key: String,
}

pub type SecretsManager = Store<Secrets, TomlCodec>;
