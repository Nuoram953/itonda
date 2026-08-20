use axum::{body::Body, http};
use http::{Request, StatusCode};
use itonda_server::config::CombinedConfig;
use tower::ServiceExt;

use crate::common::{app::test_app, response::json};

#[tokio::test]
async fn patch_config_updates_single_setting_field() {
    let app = test_app().await;

    let payload = serde_json::json!({
        "settings": {
            "metadata": {
                "steam": {
                    "enabled": false
                }
            }
        }
    });

    let response = app
        .router
        .clone()
        .oneshot(
            Request::builder()
                .uri("/config")
                .method("PATCH")
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_vec(&payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body: CombinedConfig = json(response).await;

    assert!(!body.settings.metadata.steam.enabled);
    assert!(body.settings.metadata.steam.fetch_achievements);
    assert!(body.settings.metadata.steam.fetch_playtime);
    assert_eq!(body.app.server.port, 3005);
}

#[tokio::test]
async fn patch_config_updates_secrets() {
    let app = test_app().await;

    let payload = serde_json::json!({
        "secrets": {
            "storefronts": {
                "steam": {
                    "api_key": "my-secret-token",
                    "steam_id": "12345678"
                }
            },
            "asset_store": {
                "steam_grid_db": {
                    "api_key": "sgdb-key"
                },
                "tmdb": {
                    "api_key": "tmdb-key"
                }
            }
        }
    });

    let response = app
        .router
        .clone()
        .oneshot(
            Request::builder()
                .uri("/config")
                .method("PATCH")
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_vec(&payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body: CombinedConfig = json(response).await;

    assert_eq!(body.secrets.storefronts.steam.api_key, "my-secret-token");
    assert_eq!(body.secrets.storefronts.steam.steam_id, "12345678");
    assert_eq!(body.secrets.asset_store.steam_grid_db.api_key, "sgdb-key");
    assert_eq!(body.secrets.asset_store.tmdb.api_key, "tmdb-key");
}

#[tokio::test]
async fn patch_config_updates_app_server_config() {
    let app = test_app().await;

    let payload = serde_json::json!({
        "app": {
            "server": {
                "port": 8080
            }
        }
    });

    let response = app
        .router
        .clone()
        .oneshot(
            Request::builder()
                .uri("/config")
                .method("PATCH")
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_vec(&payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body: CombinedConfig = json(response).await;
    assert_eq!(body.app.server.port, 8080);
    assert_eq!(body.app.server.host, "0.0.0.0");
}

#[tokio::test]
async fn patch_config_updates_multiple_sections_simultaneously() {
    let app = test_app().await;

    let payload = serde_json::json!({
        "settings": {
            "metadata": {
                "steam": {
                    "fetch_playtime": false
                }
            }
        },
        "app": {
            "server": {
                "host": "127.0.0.1",
                "port": 9000
            }
        },
        "secrets": {
            "storefronts": {
                "steam": {
                    "steam_id": "42"
                }
            }
        }
    });

    let response = app
        .router
        .clone()
        .oneshot(
            Request::builder()
                .uri("/config")
                .method("PATCH")
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_vec(&payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body: CombinedConfig = json(response).await;
    assert!(!body.settings.metadata.steam.fetch_playtime);
    assert!(body.settings.metadata.steam.fetch_achievements);
    assert_eq!(body.app.server.host, "127.0.0.1");

    assert_eq!(body.app.server.port, 9000);
    assert_eq!(body.secrets.storefronts.steam.steam_id, "42");
}
