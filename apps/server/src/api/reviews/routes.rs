use axum::{
    Router,
    routing::{get, put},
};

use crate::state::AppState;

use super::handlers;

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/media/{media_id}/review",
            get(handlers::get_review_overview)
                .put(handlers::upsert_review)
                .delete(handlers::delete_review),
        )
        .route(
            "/media/{media_id}/thoughts",
            get(handlers::get_thoughts).post(handlers::create_thought),
        )
        .route(
            "/media/{media_id}/thoughts/{thought_id}",
            put(handlers::update_thought).delete(handlers::delete_thought),
        )
}
