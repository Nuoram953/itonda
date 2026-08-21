use itonda_domain::{
    agents::models::Agent,
    assets::types::AssetType,
    media::{
        models::{
            Asset, Launch, Media, MediaDetails, MediaGameDetails, MediaInstallation,
            MediaStorefront, PaginatedMedia,
        },
        types::{MediaSortField, MediaSource, MediaStatus, MediaType, SortOrder},
    },
    storefronts::models::StorefrontId,
};
use utoipa::OpenApi;

use crate::api::{
    agents::schemas::GetAgentsResponse,
    media::schemas::{
        MediaImportItem, MediaImportPayload, MediaImportResponse, MediaLaunchPayload,
        MediaQueryParams, MediaRefreshPayload, MediaResponse,
    },
    response::{CommandResponse, CommandStatus, JobResponse, JobStatus},
};

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::api::media::handlers::get_media,
        crate::api::media::handlers::get_media_by_id,
        crate::api::media::handlers::import_media,
        crate::api::media::handlers::refresh,
        crate::api::media::handlers::launch_media,
        crate::api::media::handlers::update_status,
        crate::api::assets::handlers::get_asset,
        crate::api::agents::handlers::get_connected_agents,
        crate::api::config::handlers::get_config,
        crate::api::config::handlers::update_config,
        crate::api::auth::handlers::steam_login,
        crate::api::auth::handlers::steam_callback,
        crate::api::auth::handlers::steam_status,
        crate::api::auth::handlers::steam_disconnect,
    ),

    components(
        schemas(
            crate::api::auth::schemas::StorefrontAuthStatusResponse,
            crate::api::auth::schemas::AuthUrlResponse,
            crate::api::auth::schemas::AuthActionResponse,
            crate::api::auth::schemas::SteamCallbackPayload,
            MediaResponse,
            MediaQueryParams,
            MediaRefreshPayload,
            MediaLaunchPayload,
            MediaImportPayload,
            MediaImportItem,
            MediaImportResponse,
            JobResponse,
            JobStatus,
            CommandResponse,
            CommandStatus,
            GetAgentsResponse,
            Media,
            PaginatedMedia,
            Asset,
            AssetType,
            Launch,
            MediaStorefront,
            MediaInstallation,
            MediaDetails,
            MediaGameDetails,
            MediaType,
            MediaStatus,
            MediaSortField,
            SortOrder,
            MediaSource,
            StorefrontId,
            Agent,
            crate::config::CombinedConfig,
            crate::config::PatchConfigPayload,
            crate::config::Settings,
            crate::config::PatchSettings,
            crate::config::settings::MetadataSettings,
            crate::config::settings::PatchMetadataSettings,
            crate::config::settings::SteamSettings,
            crate::config::settings::PatchSteamSettings,
            crate::config::Secrets,
            crate::config::PatchSecrets,
            crate::config::secrets::StorefrontsSettings,
            crate::config::secrets::PatchStorefrontsSettings,
            crate::config::secrets::AssetStoreSettings,
            crate::config::secrets::PatchAssetStoreSettings,
            crate::config::secrets::SteamSettings,
            crate::config::secrets::SteamGridDbSettings,
            crate::config::secrets::PatchSteamGridDbSettings,
            crate::config::secrets::TheMovieDatabaseSettings,
            crate::config::secrets::PatchTheMovieDatabaseSettings,
            crate::config::AppConfig,
            crate::config::PatchAppConfig,
            crate::config::app::ServerConfig,
            crate::config::app::PatchServerConfig,
        )
    ),
    servers(
        (url="/api/v1")
    )
)]
pub struct ApiDoc;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_openapi_spec_validity() {
        let spec = ApiDoc::openapi();
        let json = spec
            .to_pretty_json()
            .expect("Failed to serialize OpenAPI spec to JSON");
        assert!(!json.is_empty());
    }
}
