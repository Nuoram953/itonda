use crate::common::{app::test_app, response::json};
use axum::http;
use http::{Request, StatusCode};
use itonda_domain::metadata::models::MediaSearchResult;
use tower::ServiceExt;

#[tokio::test]
async fn search_media_returns_empty_when_query_too_short() {
    let app = test_app().await;

    let response = app
        .router
        .oneshot(
            Request::builder()
                .uri("/media/search?type=game&query=a")
                .method("GET")
                .body(axum::body::Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let results: Vec<MediaSearchResult> = json(response).await;
    assert!(results.is_empty());
}

#[tokio::test]
async fn search_media_returns_empty_when_no_searcher_registered() {
    let app = test_app().await;

    let response = app
        .router
        .oneshot(
            Request::builder()
                .uri("/media/search?type=game&query=witcher")
                .method("GET")
                .body(axum::body::Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let results: Vec<MediaSearchResult> = json(response).await;
    assert!(results.is_empty());
}

#[tokio::test]
async fn search_media_returns_400_for_invalid_media_type() {
    let app = test_app().await;

    let response = app
        .router
        .oneshot(
            Request::builder()
                .uri("/media/search?type=invalid_type&query=test")
                .method("GET")
                .body(axum::body::Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}
