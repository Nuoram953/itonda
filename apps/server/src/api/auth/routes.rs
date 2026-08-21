use axum::{
    Router,
    routing::{get, post},
};

use crate::state::AppState;

use super::handlers::{steam_callback, steam_disconnect, steam_login, steam_status};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/auth/steam/login", get(steam_login))
        .route("/auth/steam/callback", post(steam_callback))
        .route("/auth/steam/status", get(steam_status))
        .route("/auth/steam/disconnect", post(steam_disconnect))
}
