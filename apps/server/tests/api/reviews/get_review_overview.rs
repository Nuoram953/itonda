use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use itonda_domain::reviews::{
    models::{GameReviewOverview, ReviewVerdict},
    service as ReviewService,
};
use itonda_server::api::error::{ApiError, ErrorResponse};
use tower::ServiceExt;
use uuid::Uuid;

use crate::common::{app::test_app, fixtures::media::MediaFixture, response::json};

#[tokio::test]
async fn get_review_overview_returns_empty_when_no_review_or_thoughts() {
    let app = test_app().await;
    let fixture = MediaFixture::default().insert(&app.db).await;

    let response = app
        .router
        .oneshot(
            Request::builder()
                .uri(format!("/media/{}/review", fixture.media.id))
                .method("GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let overview: GameReviewOverview = json(response).await;
    assert!(overview.review.is_none());
    assert!(overview.thoughts.is_empty());
}

#[tokio::test]
async fn get_review_overview_returns_review_and_thoughts() {
    let app = test_app().await;
    let fixture = MediaFixture::default().insert(&app.db).await;

    ReviewService::upsert_review(
        &app.db,
        &fixture.media.id,
        ReviewVerdict::Masterpiece,
        Some("Incredible campaign and worldbuilding.".to_string()),
    )
    .await
    .unwrap();

    ReviewService::create_thought(
        &app.db,
        &fixture.media.id,
        "Soundtrack in Gears".to_string(),
        "The music really surprised me.".to_string(),
        Some("audio".to_string()),
        Some(150),
    )
    .await
    .unwrap();

    let response = app
        .router
        .oneshot(
            Request::builder()
                .uri(format!("/media/{}/review", fixture.media.id))
                .method("GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let overview: GameReviewOverview = json(response).await;
    let review = overview.review.expect("review should be present");
    assert_eq!(review.media_id, fixture.media.id);
    assert_eq!(review.verdict, ReviewVerdict::Masterpiece);
    assert_eq!(
        review.summary,
        Some("Incredible campaign and worldbuilding.".into())
    );

    assert_eq!(overview.thoughts.len(), 1);
    assert_eq!(overview.thoughts[0].title, "Soundtrack in Gears");
    assert_eq!(overview.thoughts[0].category, "audio");
    assert_eq!(overview.thoughts[0].playtime_minutes, Some(150));
}

#[tokio::test]
async fn get_review_overview_returns_404_when_media_not_found() {
    let app = test_app().await;
    let missing_media_id = Uuid::new_v4().to_string();

    let response = app
        .router
        .oneshot(
            Request::builder()
                .uri(format!("/media/{missing_media_id}/review"))
                .method("GET")
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
