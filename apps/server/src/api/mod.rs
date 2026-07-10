pub mod media;
pub mod openapi;

use axum::Router;

pub fn router() -> Router {
    Router::new().merge(media::routes::router())
}
