use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use itonda_server::api::auth::schemas::{
    AuthActionResponse, AuthUrlResponse, StorefrontAuthStatusResponse,
};
use tower::ServiceExt;

use crate::common::{app::test_app, response::json};

#[tokio::test]
async fn steam_login_redirects_or_returns_url() {
    let app = test_app().await;

    // Test JSON url response when redirect=false
    let response = app
        .router
        .clone()
        .oneshot(
            Request::builder()
                .uri("/auth/steam/login?redirect=false")
                .method("GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body: AuthUrlResponse = json(response).await;
    assert!(body.url.starts_with("https://steamcommunity.com/openid/login?"));
    assert!(body.url.contains("openid.mode=checkid_setup"));

    // Test 307 redirect by default
    let redirect_response = app
        .router
        .oneshot(
            Request::builder()
                .uri("/auth/steam/login")
                .method("GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(redirect_response.status(), StatusCode::TEMPORARY_REDIRECT);
    assert!(redirect_response.headers().contains_key("location"));
}

#[tokio::test]
async fn steam_status_and_disconnect() {
    let app = test_app().await;

    // Initially not connected
    let response = app
        .router
        .clone()
        .oneshot(
            Request::builder()
                .uri("/auth/steam/status")
                .method("GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body: StorefrontAuthStatusResponse = json(response).await;
    assert!(!body.connected);
    assert_eq!(body.steam_id, None);
    assert_eq!(body.account_name, None);
    assert_eq!(body.avatar_url, None);

    // Update secrets via PATCH /config to simulate connected state with profile
    let patch_payload = serde_json::json!({
        "secrets": {
            "storefronts": {
                "steam": {
                    "steam_id": "76561198000000000",
                    "account_name": "TestGamer",
                    "avatar_url": "https://example.com/avatar.jpg"
                }
            }
        }
    });

    let patch_res = app
        .router
        .clone()
        .oneshot(
            Request::builder()
                .uri("/config")
                .method("PATCH")
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_vec(&patch_payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(patch_res.status(), StatusCode::OK);

    let connected_res = app
        .router
        .clone()
        .oneshot(
            Request::builder()
                .uri("/auth/steam/status")
                .method("GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let connected_body: StorefrontAuthStatusResponse = json(connected_res).await;
    assert!(connected_body.connected);
    assert_eq!(connected_body.steam_id, Some("76561198000000000".into()));
    assert_eq!(connected_body.account_name, Some("TestGamer".into()));
    assert_eq!(
        connected_body.avatar_url,
        Some("https://example.com/avatar.jpg".into())
    );

    // Test disconnect endpoint
    let disconnect_response = app
        .router
        .clone()
        .oneshot(
            Request::builder()
                .uri("/auth/steam/disconnect")
                .method("POST")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(disconnect_response.status(), StatusCode::OK);
    let action_body: AuthActionResponse = json(disconnect_response).await;
    assert!(action_body.success);

    // Verify status after disconnect
    let post_disconnect_res = app
        .router
        .oneshot(
            Request::builder()
                .uri("/auth/steam/status")
                .method("GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let post_disconnect_body: StorefrontAuthStatusResponse = json(post_disconnect_res).await;
    assert!(!post_disconnect_body.connected);
    assert_eq!(post_disconnect_body.account_name, None);
    assert_eq!(post_disconnect_body.avatar_url, None);
}
