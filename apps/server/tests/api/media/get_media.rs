use crate::common::{app::test_app, fixtures::media::MediaFixture, response::json};
use axum::{body::Body, http};
use http::{Request, StatusCode};
use itonda_server::api::media::schemas::MediaResponse;
use tower::ServiceExt;

#[tokio::test]
async fn get_media_returns_media_list() {
    let app = test_app().await;

    let media = MediaFixture {
        ..Default::default()
    };

    media.insert(&app.db).await;

    let response = app
        .router
        .oneshot(
            Request::builder()
                .uri("/media")
                .method("GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let response: MediaResponse = json(response).await;

    assert_eq!(response.items.len(), 1);
}

#[tokio::test]
async fn get_media_returns_empty_list() {
    let app = test_app().await;

    let response = app
        .router
        .oneshot(
            Request::builder()
                .uri("/media")
                .method("GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let response: MediaResponse = json(response).await;

    assert!(response.items.is_empty());
}
