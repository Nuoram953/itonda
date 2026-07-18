use axum::{Router, routing::get};

mod agent;
mod imports;

pub use agent::AgentManager;

pub fn router() -> Router<crate::state::AppState> {
    Router::new()
        .route("/imports", get(imports::websocket))
        .route("/agent/connect", get(agent::agent_ws))
}
