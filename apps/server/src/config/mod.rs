pub mod app;
pub mod secrets;
pub mod settings;

pub use app::{AppConfig, PatchAppConfig, PatchServerConfig};
pub use secrets::{
    AssetStoreSettings, PatchAssetStoreSettings, PatchSecrets, PatchSteamGridDbSettings,
    PatchSteamSecrets, PatchStorefrontsSettings, PatchTheMovieDatabaseSettings, Secrets,
    SteamGridDbSettings, SteamSettings as SteamSecrets, StorefrontsSettings,
    TheMovieDatabaseSettings,
};
pub use settings::{
    MetadataSettings, PatchMetadataSettings, PatchSettings, PatchSteamSettings, Settings,
    SteamSettings,
};

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::state::AppState;

#[derive(Debug, Clone, Serialize, Deserialize, Default, ToSchema, PartialEq)]
#[serde(default)]
pub struct CombinedConfig {
    pub settings: Settings,
    pub secrets: Secrets,
    pub app: AppConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, ToSchema, PartialEq)]
#[serde(default)]
pub struct PatchConfigPayload {
    pub settings: Option<PatchSettings>,
    pub secrets: Option<PatchSecrets>,
    pub app: Option<PatchAppConfig>,
}

impl CombinedConfig {
    pub async fn from_state(state: &AppState) -> Self {
        Self {
            settings: state.settings.get().await,
            secrets: state.secrets.get().await,
            app: state.config.get().await,
        }
    }
}
