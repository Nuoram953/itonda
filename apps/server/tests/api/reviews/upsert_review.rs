use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use itonda_domain::reviews::{
    models::{GameReview, ReviewVerdict},
    service as ReviewService,
};
use itonda_server::api::error::{ApiError, ErrorResponse};
use serde_json::json as json_value;
use tower::ServiceExt;
use uuid::Uuid;

use crate::common::{app::test_app, fixtures::media::MediaFixture, response::json};

#[tokio::test]
async fn upsert_review_creates_new_review() {
    let app = test_app().await;
    let fixture = MediaFixture::default().insert(&app.db).await;

    let payload = json_value!({
        "verdict": "masterpiece",
        "summary": "Incredible campaign and worldbuilding."
    });

    let response = app
        .router
        .oneshot(
            Request::builder()
                .uri(format!("/media/{}/review", fixture.media.id))
                .method("PUT")
                .header("content-type", "application/json")
                .body(Body::from(payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let review: GameReview = json(response).await;
    assert_eq!(review.media_id, fixture.media.id);
    assert_eq!(review.verdict, ReviewVerdict::Masterpiece);
    assert_eq!(
        review.summary,
        Some("Incredible campaign and worldbuilding.".into())
    );
}

#[tokio::test]
async fn upsert_review_updates_existing_review() {
    let app = test_app().await;
    let fixture = MediaFixture::default().insert(&app.db).await;

    ReviewService::upsert_review(
        &app.db,
        &fixture.media.id,
        ReviewVerdict::Masterpiece,
        Some("Initial review".to_string()),
    )
    .await
    .unwrap();

    let update_payload = json_value!({
        "verdict": "recommended",
        "summary": "Great game overall with fun co-op."
    });

    let response = app
        .router
        .oneshot(
            Request::builder()
                .uri(format!("/media/{}/review", fixture.media.id))
                .method("PUT")
                .header("content-type", "application/json")
                .body(Body::from(update_payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let updated: GameReview = json(response).await;
    assert_eq!(updated.media_id, fixture.media.id);
    assert_eq!(updated.verdict, ReviewVerdict::Recommended);
    assert_eq!(
        updated.summary,
        Some("Great game overall with fun co-op.".into())
    );
}

#[tokio::test]
async fn upsert_review_returns_404_when_media_not_found() {
    let app = test_app().await;
    let missing_media_id = Uuid::new_v4().to_string();

    let payload = json_value!({
        "verdict": "recommended",
        "summary": "Great game."
    });

    let response = app
        .router
        .oneshot(
            Request::builder()
                .uri(format!("/media/{missing_media_id}/review"))
                .method("PUT")
                .header("content-type", "application/json")
                .body(Body::from(payload.to_string()))
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

#[tokio::test]
async fn upsert_review_returns_400_when_payload_invalid() {
    let app = test_app().await;
    let fixture = MediaFixture::default().insert(&app.db).await;

    let invalid_payload = json_value!({
        "verdict": "invalid_verdict",
        "summary": "Some text"
    });

    let response = app
        .router
        .oneshot(
            Request::builder()
                .uri(format!("/media/{}/review", fixture.media.id))
                .method("PUT")
                .header("content-type", "application/json")
                .body(Body::from(invalid_payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let error: ErrorResponse = json(response).await;
    let expected = ApiError::InvalidPayload.error_body();
    assert_eq!(error.code, expected.code);
    assert_eq!(error.message, expected.message);
}
