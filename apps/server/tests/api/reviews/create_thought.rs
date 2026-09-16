use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use itonda_domain::reviews::models::GameThought;
use itonda_server::api::error::{ApiError, ErrorResponse};
use serde_json::json as json_value;
use tower::ServiceExt;
use uuid::Uuid;

use crate::common::{app::test_app, fixtures::media::MediaFixture, response::json};

#[tokio::test]
async fn create_thought_success() {
    let app = test_app().await;
    let fixture = MediaFixture::default().insert(&app.db).await;

    let payload = json_value!({
        "title": "Soundtrack in Gears",
        "content": "The music of gears really surprised me during the combat section.",
        "category": "audio",
        "playtime_minutes": 150
    });

    let response = app
        .router
        .oneshot(
            Request::builder()
                .uri(format!("/media/{}/thoughts", fixture.media.id))
                .method("POST")
                .header("content-type", "application/json")
                .body(Body::from(payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);
    let thought: GameThought = json(response).await;
    assert_eq!(thought.media_id, fixture.media.id);
    assert_eq!(thought.title, "Soundtrack in Gears");
    assert_eq!(
        thought.content,
        "The music of gears really surprised me during the combat section."
    );
    assert_eq!(thought.category, "audio");
    assert_eq!(thought.playtime_minutes, Some(150));
}

#[tokio::test]
async fn create_thought_defaults_category_to_general() {
    let app = test_app().await;
    let fixture = MediaFixture::default().insert(&app.db).await;

    let payload = json_value!({
        "title": "Fun Mechanics",
        "content": "Movement feels fluid and responsive."
    });

    let response = app
        .router
        .oneshot(
            Request::builder()
                .uri(format!("/media/{}/thoughts", fixture.media.id))
                .method("POST")
                .header("content-type", "application/json")
                .body(Body::from(payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);
    let thought: GameThought = json(response).await;
    assert_eq!(thought.title, "Fun Mechanics");
    assert_eq!(thought.category, "general");
    assert_eq!(thought.playtime_minutes, None);
}

#[tokio::test]
async fn create_thought_returns_422_when_validation_fails() {
    let app = test_app().await;
    let fixture = MediaFixture::default().insert(&app.db).await;

    let invalid_payload = json_value!({
        "title": "   ",
        "content": "Valid content",
        "category": "audio"
    });

    let response = app
        .router
        .oneshot(
            Request::builder()
                .uri(format!("/media/{}/thoughts", fixture.media.id))
                .method("POST")
                .header("content-type", "application/json")
                .body(Body::from(invalid_payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
async fn create_thought_returns_404_when_media_not_found() {
    let app = test_app().await;
    let missing_media_id = Uuid::new_v4().to_string();

    let payload = json_value!({
        "title": "Valid Title",
        "content": "Valid content"
    });

    let response = app
        .router
        .oneshot(
            Request::builder()
                .uri(format!("/media/{missing_media_id}/thoughts"))
                .method("POST")
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
