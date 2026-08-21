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

    // Test disconnect endpoint
    let disconnect_response = app
        .router
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
}
