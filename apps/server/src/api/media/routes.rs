use axum::{
    Router,
    routing::{get, patch, post},
};

use crate::state::AppState;

use super::handlers;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/media", get(handlers::get_media))
        .route("/media/{media_id}", get(handlers::get_media_by_id))
        .route(
            "/media/{media_id}/status/{status_id}",
            patch(handlers::update_status),
        )
        .route("/media/refresh", post(handlers::refresh))
        .route("/media/refresh/{media_id}", post(handlers::refresh_media_by_id))
        .route("/media/{media_id}/refresh", post(handlers::refresh_media_by_id))
        .route("/media/import", post(handlers::import_media))
        .route("/media/launch/{launch_id}", post(handlers::launch_media))
}
