use itonda_domain::store::{Store, toml::TomlCodec};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(default)]
pub struct Settings {
    pub metadata: MetadataSettings,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            metadata: MetadataSettings {
                steam: SteamSettings {
                    enabled: true,
                    fetch_achievements: true,
                    fetch_playtime: true,
                },
            },
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(default)]
pub struct MetadataSettings {
    pub steam: SteamSettings,
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
    }
}

impl MetadataSettings {
    pub fn apply_patch(&mut self, patch: PatchMetadataSettings) {
        if let Some(steam) = patch.steam {
            self.steam.apply_patch(steam);
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
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct PatchMetadataSettings {
    pub steam: Option<PatchSteamSettings>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct PatchSteamSettings {
    pub enabled: Option<bool>,
    pub fetch_achievements: Option<bool>,
    pub fetch_playtime: Option<bool>,
}
