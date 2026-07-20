use itonda_domain::store::{Store, toml::TomlCodec};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetadataSettings {
    pub steam: SteamSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SteamSettings {
    pub enabled: bool,
    pub fetch_achievements: bool,
    pub fetch_playtime: bool,
}

pub type SettingsManager = Store<Settings, TomlCodec>;
