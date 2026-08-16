use itonda_domain::{
    agents::models::Agent,
    assets::types::AssetType,
    media::{
        models::{Asset, Launch, Media, MediaDetails, MediaGameDetails, PaginatedMedia},
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
        crate::api::agents::handlers::get_connected_agents
    ),
    components(
        schemas(
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
            MediaDetails,
            MediaGameDetails,
            MediaType,
            MediaStatus,
            MediaSortField,
            SortOrder,
            MediaSource,
            StorefrontId,
            Agent,
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
