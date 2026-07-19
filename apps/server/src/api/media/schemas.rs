use itonda_domain::media::models::{Media, MediaType};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct MediaResponse {
    pub items: Vec<Media>,
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
