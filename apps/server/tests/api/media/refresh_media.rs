use crate::common::app::test_app;
use axum::{body::Body, http};
use http::{Request, StatusCode};
use itonda_server::workers::jobs::Job;
use tower::ServiceExt;

#[tokio::test]
async fn refresh_media_creates_job() {
    let mut app = test_app().await;

    let body = serde_json::json!({
        "storefront": null
    });

    let response = app
        .router
        .oneshot(
            Request::builder()
                .uri("/media/refresh")
                .method("POST")
                .header("content-type", "application/json")
                .body(Body::from(body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::ACCEPTED);

    let job = app.jobs.recv().await.unwrap();

    assert!(matches!(job, Job::Sync(_)));
}
