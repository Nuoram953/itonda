use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use itonda_domain::reviews::{models::GameThought, service as ReviewService};
use itonda_server::api::error::{ApiError, ErrorResponse};
use tower::ServiceExt;
use uuid::Uuid;

use crate::common::{app::test_app, fixtures::media::MediaFixture, response::json};

#[tokio::test]
async fn get_thoughts_returns_empty_list_when_no_thoughts() {
    let app = test_app().await;
    let fixture = MediaFixture::default().insert(&app.db).await;

    let response = app
        .router
        .oneshot(
            Request::builder()
                .uri(format!("/media/{}/thoughts", fixture.media.id))
                .method("GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let thoughts: Vec<GameThought> = json(response).await;
    assert!(thoughts.is_empty());
}

#[tokio::test]
async fn get_thoughts_returns_list_of_thoughts() {
    let app = test_app().await;
    let fixture = MediaFixture::default().insert(&app.db).await;

    let thought1 = ReviewService::create_thought(
        &app.db,
        &fixture.media.id,
        "First Impression".to_string(),
        "Loved the intro sequence.".to_string(),
        Some("gameplay".to_string()),
        Some(30),
    )
    .await
    .unwrap();

    let thought2 = ReviewService::create_thought(
        &app.db,
        &fixture.media.id,
        "Boss Fight".to_string(),
        "Second boss was super challenging.".to_string(),
        Some("bosses".to_string()),
        Some(120),
    )
    .await
    .unwrap();

    let response = app
        .router
        .oneshot(
            Request::builder()
                .uri(format!("/media/{}/thoughts", fixture.media.id))
                .method("GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let thoughts: Vec<GameThought> = json(response).await;
    assert_eq!(thoughts.len(), 2);
    let ids: Vec<&str> = thoughts.iter().map(|t| t.id.as_str()).collect();
    assert!(ids.contains(&thought1.id.as_str()));
    assert!(ids.contains(&thought2.id.as_str()));
}

#[tokio::test]
async fn get_thoughts_returns_404_when_media_not_found() {
    let app = test_app().await;
    let missing_media_id = Uuid::new_v4().to_string();

    let response = app
        .router
        .oneshot(
            Request::builder()
                .uri(format!("/media/{missing_media_id}/thoughts"))
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
