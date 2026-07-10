use axum::{routing::get, Router};

use super::handlers;

pub fn router() -> Router {
    Router::new().route("/media", get(handlers::get_media))
}
