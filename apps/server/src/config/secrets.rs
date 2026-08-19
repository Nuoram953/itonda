use itonda_domain::store::{Store, toml::TomlCodec};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(default)]
pub struct Secrets {
    pub storefronts: StorefrontsSettings,
    pub asset_store: AssetStoreSettings,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(default)]
pub struct StorefrontsSettings {
    pub steam: SteamSettings,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(default)]
pub struct AssetStoreSettings {
    pub steam_grid_db: SteamGridDbSettings,
    pub tmdb: TheMovieDatabaseSettings,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[schema(as = SteamSecrets)]
#[serde(default)]
pub struct SteamSettings {
    pub api_key: String,
    pub steam_id: u64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(default)]
pub struct SteamGridDbSettings {
    pub api_key: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(default)]
pub struct TheMovieDatabaseSettings {
    pub api_key: String,
}

pub type SecretsManager = Store<Secrets, TomlCodec>;

impl Secrets {
    pub fn apply_patch(&mut self, patch: PatchSecrets) {
        if let Some(storefronts) = patch.storefronts {
            self.storefronts.apply_patch(storefronts);
        }
        if let Some(asset_store) = patch.asset_store {
            self.asset_store.apply_patch(asset_store);
        }
    }
}

fn apply_secret_string(target: &mut String, value: Option<String>) {
    if let Some(v) = value {
        *target = v;
    }
}

impl StorefrontsSettings {
    pub fn apply_patch(&mut self, patch: PatchStorefrontsSettings) {
        if let Some(steam) = patch.steam {
            self.steam.apply_patch(steam);
        }
    }
}

impl SteamSettings {
    pub fn apply_patch(&mut self, patch: PatchSteamSecrets) {
        apply_secret_string(&mut self.api_key, patch.api_key);
        if let Some(steam_id) = patch.steam_id {
            self.steam_id = steam_id;
        }
    }
}

impl AssetStoreSettings {
    pub fn apply_patch(&mut self, patch: PatchAssetStoreSettings) {
        if let Some(steam_grid_db) = patch.steam_grid_db {
            self.steam_grid_db.apply_patch(steam_grid_db);
        }
        if let Some(tmdb) = patch.tmdb {
            self.tmdb.apply_patch(tmdb);
        }
    }
}

impl SteamGridDbSettings {
    pub fn apply_patch(&mut self, patch: PatchSteamGridDbSettings) {
        apply_secret_string(&mut self.api_key, patch.api_key);
    }
}

impl TheMovieDatabaseSettings {
    pub fn apply_patch(&mut self, patch: PatchTheMovieDatabaseSettings) {
        apply_secret_string(&mut self.api_key, patch.api_key);
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct PatchSecrets {
    pub storefronts: Option<PatchStorefrontsSettings>,
    pub asset_store: Option<PatchAssetStoreSettings>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct PatchStorefrontsSettings {
    pub steam: Option<PatchSteamSecrets>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[schema(as = PatchSteamSecrets)]
pub struct PatchSteamSecrets {
    pub api_key: Option<String>,
    pub steam_id: Option<u64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct PatchAssetStoreSettings {
    pub steam_grid_db: Option<PatchSteamGridDbSettings>,
    pub tmdb: Option<PatchTheMovieDatabaseSettings>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct PatchSteamGridDbSettings {
    pub api_key: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct PatchTheMovieDatabaseSettings {
    pub api_key: Option<String>,
}
