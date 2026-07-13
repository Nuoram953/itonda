use axum::{Router, routing::get};

mod imports;

pub fn router() -> Router<crate::state::AppState> {
    Router::new().route("/imports", get(imports::websocket))
}
