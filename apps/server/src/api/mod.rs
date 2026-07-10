pub mod error;
pub mod media;
pub mod openapi;
pub mod response;

use axum::Router;

use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new().merge(media::routes::router())
}
