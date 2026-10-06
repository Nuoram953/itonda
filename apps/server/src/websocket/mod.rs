use axum::{Router, routing::get};

mod agent;
mod events;

pub use agent::AgentManager;

pub fn router() -> Router<crate::state::AppState> {
    Router::new()
        .route("/", get(events::websocket))
        .route("/agent/connect", get(agent::agent_ws))
}
