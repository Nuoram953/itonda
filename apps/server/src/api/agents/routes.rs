use axum::{Router, routing::get};

use crate::state::AppState;

use super::handlers;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/agents", get(handlers::get_connected_agents))
        .route("/agents/fs", get(handlers::list_agent_files))
}
