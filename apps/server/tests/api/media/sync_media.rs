use crate::common::app::test_app;
use axum::{body::Body, http};
use http::{Request, StatusCode};
use itonda_server::workers::jobs::Job;
use tower::ServiceExt;

#[tokio::test]
async fn sync_media_by_id_creates_job() {
    let mut app = test_app().await;

    let response = app
        .router
        .oneshot(
            Request::builder()
                .uri("/media/test-media-123/sync")
                .method("POST")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::ACCEPTED);

    let job = app.jobs.recv().await.unwrap();

    match job {
        Job::Sync(sync_job) => {
            assert_eq!(sync_job.media_id, Some("test-media-123".to_string()));
            assert!(sync_job.force);
        }
        _ => panic!("expected Job::Sync"),
    }
}

#[tokio::test]
async fn sync_media_by_id_with_payload_creates_job() {
    let mut app = test_app().await;

    let body = serde_json::json!({
        "force": false
    });

    let response = app
        .router
        .oneshot(
            Request::builder()
                .uri("/media/test-media-123/sync")
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
            assert_eq!(sync_job.media_id, Some("test-media-123".to_string()));
            assert!(!sync_job.force);
        }
        _ => panic!("expected Job::Sync"),
    }
}
