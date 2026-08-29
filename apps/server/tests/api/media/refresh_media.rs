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

    assert!(matches!(job, Job::Sync(sync_job) if sync_job.media_id.is_none()));
}

#[tokio::test]
async fn refresh_single_media_creates_job_with_media_id() {
    let mut app = test_app().await;

    let body = serde_json::json!({
        "force": true
    });

    let response = app
        .router
        .oneshot(
            Request::builder()
                .uri("/media/refresh/game-123")
                .method("POST")
                .header("content-type", "application/json")
                .body(Body::from(body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::ACCEPTED);

    let job = app.jobs.recv().await.unwrap();

    match job {
        Job::Sync(sync_job) => {
            assert_eq!(sync_job.media_id.as_deref(), Some("game-123"));
            assert!(sync_job.force);
        }
        _ => panic!("Expected Job::Sync"),
    }
}

#[tokio::test]
async fn refresh_single_media_with_media_id_in_middle_path() {
    let mut app = test_app().await;

    let response = app
        .router
        .oneshot(
            Request::builder()
                .uri("/media/game-456/refresh")
                .method("POST")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::json!({}).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::ACCEPTED);

    let job = app.jobs.recv().await.unwrap();

    match job {
        Job::Sync(sync_job) => {
            assert_eq!(sync_job.media_id.as_deref(), Some("game-456"));
            assert!(!sync_job.force);
        }
        _ => panic!("Expected Job::Sync"),
    }
}
