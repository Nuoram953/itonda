use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use itonda_domain::reviews::service as ReviewService;
use itonda_server::api::error::{ApiError, ErrorResponse};
use tower::ServiceExt;
use uuid::Uuid;

use crate::common::{app::test_app, fixtures::media::MediaFixture, response::json};

#[tokio::test]
async fn delete_thought_success() {
    let app = test_app().await;
    let fixture = MediaFixture::default().insert(&app.db).await;

    let thought = ReviewService::create_thought(
        &app.db,
        &fixture.media.id,
        "Temporary Thought".to_string(),
        "Content to be deleted".to_string(),
        None,
        None,
    )
    .await
    .unwrap();

    let response = app
        .router
        .oneshot(
            Request::builder()
                .uri(format!(
                    "/media/{}/thoughts/{}",
                    fixture.media.id, thought.id
                ))
                .method("DELETE")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    let thoughts = ReviewService::get_thoughts(&app.db, &fixture.media.id)
        .await
        .unwrap();
    assert!(thoughts.is_empty());
}

#[tokio::test]
async fn delete_thought_returns_404_when_thought_not_found() {
    let app = test_app().await;
    let fixture = MediaFixture::default().insert(&app.db).await;
    let missing_thought_id = Uuid::new_v4().to_string();

    let response = app
        .router
        .oneshot(
            Request::builder()
                .uri(format!(
                    "/media/{}/thoughts/{missing_thought_id}",
                    fixture.media.id
                ))
                .method("DELETE")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    let error: ErrorResponse = json(response).await;
    let expected = ApiError::not_found("Thought not found").error_body();
    assert_eq!(error.code, expected.code);
    assert_eq!(error.message, expected.message);
}
