pub mod agents;
pub mod assets;
pub mod error;
pub mod extractor;
pub mod media;
pub mod middleware;
pub mod openapi;
pub mod response;

use axum::Router;

use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .merge(media::routes::router())
        .merge(assets::routes::router())
        .merge(agents::routes::router())
        .layer(axum::middleware::from_fn(
            middleware::api_logging_middleware,
        ))
}
