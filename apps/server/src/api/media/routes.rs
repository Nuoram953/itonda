use axum::{Router, routing::get, routing::post};

use crate::state::AppState;

use super::handlers;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/media", get(handlers::get_media))
        .route("/media/import", post(handlers::import_media))
}
