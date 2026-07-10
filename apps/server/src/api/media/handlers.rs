use axum::Json;

use super::schemas::MediaResponse;

#[utoipa::path(
    get,
    path = "/media",
    responses(
        (
            status = 200,
            description = "Returns media information",
            body = MediaResponse
        )
    )
)]
pub async fn get_media() -> Json<MediaResponse> {
    Json(MediaResponse {
        message: "Hello from media endpoint".to_string(),
    })
}
