use axum::{body::Body, http};
use http::{Request, StatusCode};
use itonda_server::config::CombinedConfig;
use tower::ServiceExt;

use crate::common::{app::test_app, response::json};

#[tokio::test]
async fn get_config_returns_combined_config() {
    let app = test_app().await;

    let response = app
        .router
        .oneshot(
            Request::builder()
                .uri("/config")
                .method("GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body: CombinedConfig = json(response).await;

    assert!(body.settings.metadata.steam.enabled);
    assert!(body.settings.metadata.steam.fetch_achievements);
    assert!(body.settings.metadata.steam.fetch_playtime);

    assert_eq!(body.app.server.host, "0.0.0.0");
    assert_eq!(body.app.server.port, 3005);

    assert_eq!(body.secrets.storefronts.steam.api_key, "");
    assert_eq!(body.secrets.asset_store.steam_grid_db.api_key, "");
    assert_eq!(body.secrets.asset_store.tmdb.api_key, "");
}
