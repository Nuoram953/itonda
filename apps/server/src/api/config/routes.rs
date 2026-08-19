use axum::{Router, routing::get};

use crate::state::AppState;

use super::handlers;

pub fn router() -> Router<AppState> {
    Router::new().route(
        "/config",
        get(handlers::get_config).patch(handlers::update_config),
    )
}
