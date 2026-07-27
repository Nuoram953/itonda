use crate::common::{app::test_app, fixtures::media::MediaFixture, response::json};
use axum::http;
use http::{Request, StatusCode};
use itonda_domain::protocol::message::AgentMessage;
use itonda_server::api::error::{ApiError, ErrorResponse};
use tower::ServiceExt;
use uuid::Uuid;

#[tokio::test]
async fn launch_media_creates_job() {
    let mut app = test_app().await;

    let fixture = MediaFixture::default().insert(&app.db).await;

    let response = app
        .router
        .oneshot(
            Request::builder()
                .uri(format!("/media/launch/{}", fixture.launch.unwrap().id))
                .method("POST")
                .header("content-type", "application/json")
                .body(axum::body::Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::ACCEPTED);

    let command = app
        .agent_messages
        .recv()
        .await
        .expect("agent did not receive launch command");

    assert!(matches!(command, AgentMessage::Launch(_)));
}

#[tokio::test]
async fn launch_media_returns_404_for_invalid_launch_id() {
    let app = test_app().await;

    let response = app
        .router
        .oneshot(
            Request::builder()
                .uri(format!("/media/launch/{}", Uuid::new_v4()))
                .method("POST")
                .header("content-type", "application/json")
                .body(axum::body::Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    let error: ErrorResponse = json(response).await;

    let expected = ApiError::LaunchNotFound.error_body();

    assert_eq!(error.code, expected.code);

    assert_eq!(error.message, expected.message);
}
