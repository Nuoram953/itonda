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
    assert_eq!(response.total, 1);
    assert_eq!(response.page, 1);
    assert_eq!(response.limit, 24);
    assert_eq!(response.total_pages, 1);
    assert!(!response.has_next);
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
    assert_eq!(response.total, 0);
    assert_eq!(response.total_pages, 1);
    assert!(!response.has_next);
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
    assert_eq!(body.total, 1);
    assert_eq!(body.items[0].title, "Halo");

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
    assert_eq!(body.total, 1);
    assert_eq!(body.items[0].title, "The Matrix");

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
    assert_eq!(body.total, 0);
}

#[tokio::test]
async fn get_media_supports_search_sorting_and_pagination() {
    let app = test_app().await;

    let fixture1 = MediaFixture {
        title: "Alpha Game".into(),
        media_type: "game".into(),
        ..Default::default()
    };
    fixture1.insert(&app.db).await;

    let fixture2 = MediaFixture {
        title: "Beta Game".into(),
        media_type: "game".into(),
        ..Default::default()
    };
    fixture2.insert(&app.db).await;

    let response = app
        .router
        .clone()
        .oneshot(
            Request::builder()
                .uri("/media?page=1&limit=1&sort_by=title&sort_order=asc")
                .method("GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body: MediaResponse = json(response).await;
    assert_eq!(body.items.len(), 1);
    assert_eq!(body.total, 2);
    assert_eq!(body.page, 1);
    assert_eq!(body.limit, 1);
    assert_eq!(body.total_pages, 2);
    assert!(body.has_next);
    assert_eq!(body.items[0].title, "Alpha Game");

    let response = app
        .router
        .clone()
        .oneshot(
            Request::builder()
                .uri("/media?page=2&limit=1&sort_by=title&sort_order=asc")
                .method("GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body: MediaResponse = json(response).await;
    assert_eq!(body.items.len(), 1);
    assert_eq!(body.total, 2);
    assert_eq!(body.page, 2);
    assert!(!body.has_next);
    assert_eq!(body.items[0].title, "Beta Game");

    let response = app
        .router
        .oneshot(
            Request::builder()
                .uri("/media?search=beta")
                .method("GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body: MediaResponse = json(response).await;
    assert_eq!(body.items.len(), 1);
    assert_eq!(body.total, 1);
    assert_eq!(body.items[0].title, "Beta Game");
}
