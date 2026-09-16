use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use itonda_domain::reviews::{models::ReviewVerdict, service as ReviewService};
use itonda_server::api::error::{ApiError, ErrorResponse};
use tower::ServiceExt;
use uuid::Uuid;

use crate::common::{app::test_app, fixtures::media::MediaFixture, response::json};

#[tokio::test]
async fn delete_review_removes_existing_review() {
    let app = test_app().await;
    let fixture = MediaFixture::default().insert(&app.db).await;

    ReviewService::upsert_review(
        &app.db,
        &fixture.media.id,
        ReviewVerdict::Masterpiece,
        Some("To be deleted".to_string()),
    )
    .await
    .unwrap();

    let response = app
        .router
        .oneshot(
            Request::builder()
                .uri(format!("/media/{}/review", fixture.media.id))
                .method("DELETE")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    let review = ReviewService::get_review(&app.db, &fixture.media.id)
        .await
        .unwrap();
    assert!(review.is_none());
}

#[tokio::test]
async fn delete_review_succeeds_when_no_review_exists() {
    let app = test_app().await;
    let fixture = MediaFixture::default().insert(&app.db).await;

    let response = app
        .router
        .oneshot(
            Request::builder()
                .uri(format!("/media/{}/review", fixture.media.id))
                .method("DELETE")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}

#[tokio::test]
async fn delete_review_returns_404_when_media_not_found() {
    let app = test_app().await;
    let missing_media_id = Uuid::new_v4().to_string();

    let response = app
        .router
        .oneshot(
            Request::builder()
                .uri(format!("/media/{missing_media_id}/review"))
                .method("DELETE")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    let error: ErrorResponse = json(response).await;
    let expected = ApiError::not_found("Media not found").error_body();
    assert_eq!(error.code, expected.code);
    assert_eq!(error.message, expected.message);
}
