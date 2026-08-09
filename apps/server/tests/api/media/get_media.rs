use crate::common::{app::test_app, fixtures::media::MediaFixture, response::json};
use axum::{body::Body, http};
use http::{Request, StatusCode};
use itonda_server::api::media::schemas::MediaResponse;
use tower::ServiceExt;

#[tokio::test]
async fn get_media_returns_media_list() {
    let app = test_app().await;

    let media = MediaFixture {
        ..Default::default()
    };

    media.insert(&app.db).await;

    let response = app
        .router
        .oneshot(
            Request::builder()
                .uri("/media")
                .method("GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let response: MediaResponse = json(response).await;

    assert_eq!(response.items.len(), 1);
}

#[tokio::test]
async fn get_media_returns_empty_list() {
    let app = test_app().await;

    let response = app
        .router
        .oneshot(
            Request::builder()
                .uri("/media")
                .method("GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let response: MediaResponse = json(response).await;

    assert!(response.items.is_empty());
}

#[tokio::test]
async fn get_media_filters_by_type() {
    let app = test_app().await;

    let game_fixture = MediaFixture {
        title: "Halo".into(),
        media_type: "game".into(),
        ..Default::default()
    };
    game_fixture.insert(&app.db).await;

    let movie_fixture = MediaFixture {
        title: "The Matrix".into(),
        media_type: "movie".into(),
        ..Default::default()
    };
    movie_fixture.insert(&app.db).await;

    // Filter by type=game
    let response = app
        .router
        .clone()
        .oneshot(
            Request::builder()
                .uri("/media?type=game")
                .method("GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body: MediaResponse = json(response).await;
    assert_eq!(body.items.len(), 1);
    assert_eq!(body.items[0].title, "Halo");

    // Filter by type=movie
    let response = app
        .router
        .clone()
        .oneshot(
            Request::builder()
                .uri("/media?type=movie")
                .method("GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body: MediaResponse = json(response).await;
    assert_eq!(body.items.len(), 1);
    assert_eq!(body.items[0].title, "The Matrix");

    // Filter by type=tv_show (none match)
    let response = app
        .router
        .oneshot(
            Request::builder()
                .uri("/media?type=tv_show")
                .method("GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body: MediaResponse = json(response).await;
    assert!(body.items.is_empty());
}
