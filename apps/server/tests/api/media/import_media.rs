use crate::common::{app::test_app, response::json};
use axum::{body::Body, http};
use http::{Request, StatusCode};
use itonda_server::{
    api::error::{ApiError, ErrorResponse},
    workers::jobs::Job,
};
use tower::ServiceExt;

#[tokio::test]
async fn import_media_creates_job() {
    let mut app = test_app().await;

    let body = serde_json::json!({
        "items": [
            {
                "media_type": "game",
                "title": "Elden Ring",
                "year": 2022
            }
        ]
    });

    let response = app
        .router
        .oneshot(
            Request::builder()
                .uri("/media/import")
                .method("POST")
                .header("content-type", "application/json")
                .body(Body::from(body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::ACCEPTED);

    let job = app.jobs.recv().await.unwrap();

    assert!(matches!(job, Job::Import(_)));
}

#[tokio::test]
async fn import_media_returns_422_when_required_field_is_missing() {
    let app = test_app().await;

    let body = serde_json::json!({
        "items": [
            {
                "media_type": "game",
                "year": 2022
            },
            {
                "media_type": "game",
                "title": "test",
                "year": 2022
            }
        ]
    });

    let response = app
        .router
        .oneshot(
            Request::builder()
                .uri("/media/import")
                .method("POST")
                .header("content-type", "application/json")
                .body(Body::from(body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let error: ErrorResponse = json(response).await;

    let expected = ApiError::InvalidPayload.response();

    assert_eq!(error.code, expected.code);

    assert_eq!(error.message, expected.message);
}
