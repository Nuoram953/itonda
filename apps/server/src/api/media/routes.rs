use axum::{Router, routing::get, routing::post};

use crate::state::AppState;

use super::handlers;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/media", get(handlers::get_media))
        .route("/media/refresh", post(handlers::refresh))
        .route("/media/import", post(handlers::import_media))
        .route("/media/launch/{launch_id}", post(handlers::launch_media))
}
