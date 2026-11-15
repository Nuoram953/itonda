use axum::{
    Router,
    routing::{get, patch},
};

use crate::state::AppState;

use super::handlers;

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/emulators",
            get(handlers::get_emulators).post(handlers::create_emulator),
        )
        .route(
            "/emulators/{emulator_id}",
            patch(handlers::update_emulator).delete(handlers::delete_emulator),
        )
}
