use itonda_domain::{
    media::{models::Media, types::MediaType},
    storefronts::models::StorefrontId,
};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Deserialize, ToSchema, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct MediaQueryParams {
    #[serde(rename = "type")]
    pub media_type: Option<MediaType>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct MediaResponse {
    pub items: Vec<Media>,
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
