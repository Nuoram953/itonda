use crate::common::{app::test_app, fixtures::media::MediaFixture};
use axum::{body::Body, http};
use http::{Request, StatusCode};
use itonda_database::media::{find_media_by_id, find_media_status_history};
use itonda_domain::media::types::MediaStatus;
use tower::ServiceExt;
use uuid::Uuid;

#[tokio::test]
async fn patch_media_status_updates_status_successfully() {
    let app = test_app().await;

    let fixture = MediaFixture::default().insert(&app.db).await;

    let response = app
        .router
        .oneshot(
            Request::builder()
                .uri(format!("/media/{}/status/in_progress", fixture.media.id))
                .method("PATCH")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    let updated_media = find_media_by_id(&app.db, fixture.media.id.clone())
        .await
        .unwrap();
    assert_eq!(updated_media.status_id, MediaStatus::InProgress.id());

    let history = find_media_status_history(&app.db, &fixture.media.id)
        .await
        .unwrap();
    assert_eq!(history.len(), 1);
    assert_eq!(history[0].status_id, MediaStatus::InProgress.id());
}

#[tokio::test]
async fn patch_media_status_supports_various_statuses() {
    let app = test_app().await;

    let fixture = MediaFixture::default().insert(&app.db).await;

    let statuses = [
        ("completed", MediaStatus::Completed),
        ("paused", MediaStatus::Paused),
        ("abandoned", MediaStatus::Abandoned),
        ("not_started", MediaStatus::NotStarted),
    ];

    for (status_str, expected_status) in statuses {
        let response = app
            .router
            .clone()
            .oneshot(
                Request::builder()
                    .uri(format!("/media/{}/status/{}", fixture.media.id, status_str))
                    .method("PATCH")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NO_CONTENT);

        let updated_media = find_media_by_id(&app.db, fixture.media.id.clone())
            .await
            .unwrap();
        assert_eq!(updated_media.status_id, expected_status.id());
    }
}

#[tokio::test]
async fn patch_media_status_returns_400_for_invalid_status() {
    let app = test_app().await;

    let fixture = MediaFixture::default().insert(&app.db).await;

    let response = app
        .router
        .oneshot(
            Request::builder()
                .uri(format!("/media/{}/status/invalid_status", fixture.media.id))
                .method("PATCH")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn patch_media_status_returns_error_for_nonexistent_media() {
    let app = test_app().await;

    let response = app
        .router
        .oneshot(
            Request::builder()
                .uri(format!("/media/{}/status/completed", Uuid::new_v4()))
                .method("PATCH")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}
