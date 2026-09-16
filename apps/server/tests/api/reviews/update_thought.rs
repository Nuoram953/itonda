use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use itonda_domain::reviews::{models::GameThought, service as ReviewService};
use itonda_server::api::error::{ApiError, ErrorResponse};
use serde_json::json as json_value;
use tower::ServiceExt;
use uuid::Uuid;

use crate::common::{app::test_app, fixtures::media::MediaFixture, response::json};

#[tokio::test]
async fn update_thought_success() {
    let app = test_app().await;
    let fixture = MediaFixture::default().insert(&app.db).await;

    let thought = ReviewService::create_thought(
        &app.db,
        &fixture.media.id,
        "Soundtrack in Gears".to_string(),
        "Initial thoughts on soundtrack.".to_string(),
        Some("audio".to_string()),
        Some(100),
    )
    .await
    .unwrap();

    let update_payload = json_value!({
        "title": "Orchestral Score",
        "content": "Updated thoughts on the incredible soundtrack.",
        "category": "soundtrack"
    });

    let response = app
        .router
        .oneshot(
            Request::builder()
                .uri(format!(
                    "/media/{}/thoughts/{}",
                    fixture.media.id, thought.id
                ))
                .method("PUT")
                .header("content-type", "application/json")
                .body(Body::from(update_payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let updated: GameThought = json(response).await;
    assert_eq!(updated.id, thought.id);
    assert_eq!(updated.title, "Orchestral Score");
    assert_eq!(
        updated.content,
        "Updated thoughts on the incredible soundtrack."
    );
    assert_eq!(updated.category, "soundtrack");
}

#[tokio::test]
async fn update_thought_returns_422_when_validation_fails() {
    let app = test_app().await;
    let fixture = MediaFixture::default().insert(&app.db).await;

    let thought = ReviewService::create_thought(
        &app.db,
        &fixture.media.id,
        "Initial Title".to_string(),
        "Initial content.".to_string(),
        None,
        None,
    )
    .await
    .unwrap();

    let invalid_payload = json_value!({
        "title": "   ",
        "content": "Updated content"
    });

    let response = app
        .router
        .oneshot(
            Request::builder()
                .uri(format!(
                    "/media/{}/thoughts/{}",
                    fixture.media.id, thought.id
                ))
                .method("PUT")
                .header("content-type", "application/json")
                .body(Body::from(invalid_payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
async fn update_thought_returns_404_when_thought_not_found() {
    let app = test_app().await;
    let fixture = MediaFixture::default().insert(&app.db).await;
    let missing_thought_id = Uuid::new_v4().to_string();

    let payload = json_value!({
        "title": "Valid Title",
        "content": "Valid content"
    });

    let response = app
        .router
        .oneshot(
            Request::builder()
                .uri(format!(
                    "/media/{}/thoughts/{missing_thought_id}",
                    fixture.media.id
                ))
                .method("PUT")
                .header("content-type", "application/json")
                .body(Body::from(payload.to_string()))
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
