use itonda_domain::store::{Store, toml::TomlCodec};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, ToSchema)]
#[serde(default)]
pub struct Settings {
    pub metadata: MetadataSettings,
    pub assets: AssetsSettings,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(default)]
pub struct MetadataSettings {
    pub steam: SteamSettings,
    pub igdb: TheInternetGameDatabaseMetadataSettings,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[schema(as = TheInternetGameDatabaseMetadataSettings)]
#[serde(default)]
pub struct TheInternetGameDatabaseMetadataSettings {
    pub enabled: bool,
}

impl Default for TheInternetGameDatabaseMetadataSettings {
    fn default() -> Self {
        Self { enabled: true }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(default)]
pub struct AssetsSettings {
    pub steam_grid_db: SteamGridDbSettings,
    pub tmdb: TheMovieDatabaseAssetSettings,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[schema(as = SteamGridDbAssetSettings)]
#[serde(default)]
pub struct SteamGridDbSettings {
    pub enabled: bool,
}

impl Default for SteamGridDbSettings {
    fn default() -> Self {
        Self { enabled: true }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[schema(as = TheMovieDatabaseAssetSettings)]
#[serde(default)]
pub struct TheMovieDatabaseAssetSettings {
    pub enabled: bool,
}

impl Default for TheMovieDatabaseAssetSettings {
    fn default() -> Self {
        Self { enabled: true }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(default)]
pub struct SteamSettings {
    pub enabled: bool,
    pub fetch_achievements: bool,
    pub fetch_playtime: bool,
}

impl Default for SteamSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            fetch_achievements: true,
            fetch_playtime: true,
        }
    }
}

pub type SettingsManager = Store<Settings, TomlCodec>;

impl Settings {
    pub fn apply_patch(&mut self, patch: PatchSettings) {
        if let Some(metadata) = patch.metadata {
            self.metadata.apply_patch(metadata);
        }
        if let Some(assets) = patch.assets {
            self.assets.apply_patch(assets);
        }
    }
}

impl MetadataSettings {
    pub fn apply_patch(&mut self, patch: PatchMetadataSettings) {
        if let Some(steam) = patch.steam {
            self.steam.apply_patch(steam);
        }
        if let Some(igdb) = patch.igdb {
            self.igdb.apply_patch(igdb);
        }
    }
}

impl AssetsSettings {
    pub fn apply_patch(&mut self, patch: PatchAssetsSettings) {
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
        if let Some(enabled) = patch.enabled {
            self.enabled = enabled;
        }
    }
}

impl TheMovieDatabaseAssetSettings {
    pub fn apply_patch(&mut self, patch: PatchTheMovieDatabaseAssetSettings) {
        if let Some(enabled) = patch.enabled {
            self.enabled = enabled;
        }
    }
}

impl TheInternetGameDatabaseMetadataSettings {
    pub fn apply_patch(&mut self, patch: PatchTheInternetGameDatabaseMetadataSettings) {
        if let Some(enabled) = patch.enabled {
            self.enabled = enabled;
        }
    }
}

impl SteamSettings {
    pub fn apply_patch(&mut self, patch: PatchSteamSettings) {
        if let Some(enabled) = patch.enabled {
            self.enabled = enabled;
        }
        if let Some(fetch_achievements) = patch.fetch_achievements {
            self.fetch_achievements = fetch_achievements;
        }
        if let Some(fetch_playtime) = patch.fetch_playtime {
            self.fetch_playtime = fetch_playtime;
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct PatchSettings {
    pub metadata: Option<PatchMetadataSettings>,
    pub assets: Option<PatchAssetsSettings>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct PatchMetadataSettings {
    pub steam: Option<PatchSteamSettings>,
    pub igdb: Option<PatchTheInternetGameDatabaseMetadataSettings>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct PatchAssetsSettings {
    pub steam_grid_db: Option<PatchSteamGridDbSettings>,
    pub tmdb: Option<PatchTheMovieDatabaseAssetSettings>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[schema(as = PatchSteamGridDbAssetSettings)]
pub struct PatchSteamGridDbSettings {
    pub enabled: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[schema(as = PatchTheMovieDatabaseAssetSettings)]
pub struct PatchTheMovieDatabaseAssetSettings {
    pub enabled: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[schema(as = PatchTheInternetGameDatabaseMetadataSettings)]
pub struct PatchTheInternetGameDatabaseMetadataSettings {
    pub enabled: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct PatchSteamSettings {
    pub enabled: Option<bool>,
    pub fetch_achievements: Option<bool>,
    pub fetch_playtime: Option<bool>,
}
