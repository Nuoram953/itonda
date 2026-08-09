use itonda_domain::{
    media::{
        models::Media,
        types::{MediaSortField, MediaStatus, MediaType, SortOrder},
    },
    storefronts::models::StorefrontId,
};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Deserialize, ToSchema, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct MediaQueryParams {
    #[serde(rename = "type")]
    pub media_type: Option<MediaType>,
    pub search: Option<String>,
    pub status: Option<MediaStatus>,
    pub storefront: Option<String>,
    pub sort_by: Option<MediaSortField>,
    pub sort_order: Option<SortOrder>,
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct MediaResponse {
    pub items: Vec<Media>,
    pub total: u64,
    pub page: u32,
    pub limit: u32,
    pub total_pages: u32,
    pub has_next: bool,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct MediaRefreshPayload {
    pub storefront: Option<StorefrontId>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct MediaLaunchPayload {
    pub launch_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct MediaImportPayload {
    pub items: Vec<MediaImportItem>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct MediaImportItem {
    pub title: String,
    pub media_type: MediaType,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct MediaImportResponse {
    pub message: String,
}
