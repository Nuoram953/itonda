use crate::common::{app::test_app, fixtures::media::MediaFixture, response::json};
use axum::{body::Body, http};
use http::{Request, StatusCode};
use itonda_domain::media::models::Media;
use itonda_server::api::error::{ApiError, ErrorResponse};
use tower::ServiceExt;
use uuid::Uuid;

#[tokio::test]
async fn returns_200_valid_request() {
    let app = test_app().await;

    let media = MediaFixture {
        ..Default::default()
    };

    let fixture = media.insert(&app.db).await;

    let response = app
        .router
        .oneshot(
            Request::builder()
                .uri(format!("/media/{}", fixture.media.id))
                .method("GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let response: Media = json(response).await;

    assert_eq!(response.id, fixture.media.id);
}

#[tokio::test]
async fn returns_404_when_media_not_exist() {
    let app = test_app().await;

    let response = app
        .router
        .oneshot(
            Request::builder()
                .uri(format!("/media/{}", Uuid::new_v4()))
                .method("GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    let error: ErrorResponse = json(response).await;

    let expected = ApiError::MediaNotFound.error_body();

    assert_eq!(error.code, expected.code);

    assert_eq!(error.message, expected.message);
}
