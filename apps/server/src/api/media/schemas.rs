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

#[derive(Debug, Default, Serialize, Deserialize, ToSchema)]
pub struct MediaSyncPayload {
    pub storefront: Option<StorefrontId>,
    #[serde(default)]
    pub force: bool,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct MediaItemSyncPayload {
    #[serde(default = "default_force")]
    pub force: bool,
}

fn default_force() -> bool {
    true
}

pub type MediaRefreshPayload = MediaSyncPayload;

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

#[derive(Debug, Deserialize, ToSchema, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct MediaSearchQueryParams {
    pub query: String,
    #[serde(rename = "type")]
    pub media_type: MediaType,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateMediaPayload {
    pub title: String,
    pub media_type: MediaType,
    pub external_id: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateMediaLaunchPayload {
    pub name: String,
    pub launch_type: String,
    pub emulator_id: Option<String>,
    pub game_location: Option<String>,
    #[serde(default)]
    pub program: String,
    #[serde(default)]
    pub arguments: Vec<String>,
    #[serde(default = "default_argument_mode")]
    pub argument_mode: String,
    pub working_directory: Option<String>,
    pub agent_id: Option<String>,
    #[serde(default)]
    pub is_default: bool,
    #[serde(default = "default_enabled")]
    pub enabled: bool,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateMediaLaunchPayload {
    pub name: Option<String>,
    pub launch_type: Option<String>,
    pub emulator_id: Option<Option<String>>,
    pub game_location: Option<Option<String>>,
    pub program: Option<String>,
    pub arguments: Option<Vec<String>>,
    pub argument_mode: Option<String>,
    pub working_directory: Option<Option<String>>,
    pub agent_id: Option<Option<String>>,
    pub is_default: Option<bool>,
    pub enabled: Option<bool>,
}

fn default_argument_mode() -> String {
    "append".to_string()
}

fn default_enabled() -> bool {
    true
}
