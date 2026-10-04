use crate::common::{app::test_app, response::json};
use axum::{body::Body, http};
use http::{Request, StatusCode};
use itonda_database::media::find_external_ids_by_media_ids;
use itonda_domain::media::{models::Media, types::MediaType};
use itonda_server::workers::jobs::Job;
use tower::ServiceExt;

#[tokio::test]
async fn create_media_creates_media_record_and_sync_job() {
    let mut app = test_app().await;

    let body = serde_json::json!({
        "title": "The Witcher 3",
        "media_type": "game",
        "external_id": "1942"
    });

    let response = app
        .router
        .oneshot(
            Request::builder()
                .uri("/media")
                .method("POST")
                .header("content-type", "application/json")
                .body(Body::from(body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);
    let created: Media = json(response).await;
    assert_eq!(created.title, "The Witcher 3");
    assert_eq!(created.media_type, MediaType::Game);

    let ext_ids = find_external_ids_by_media_ids(&app.db, std::slice::from_ref(&created.id))
        .await
        .unwrap();
    assert_eq!(ext_ids.len(), 1);
    assert_eq!(ext_ids[0].provider, "igdb");
    assert_eq!(ext_ids[0].external_id, "1942");

    let job = app.jobs.recv().await.unwrap();
    match job {
        Job::Sync(sync_job) => {
            assert_eq!(sync_job.media_id, Some(created.id));
            assert_eq!(sync_job.storefront, None);
            assert!(!sync_job.force);
        }
        _ => panic!("expected Job::Sync"),
    }
}

#[tokio::test]
async fn create_media_without_external_id_succeeds() {
    let mut app = test_app().await;

    let body = serde_json::json!({
        "title": "Custom Movie",
        "media_type": "movie"
    });

    let response = app
        .router
        .oneshot(
            Request::builder()
                .uri("/media")
                .method("POST")
                .header("content-type", "application/json")
                .body(Body::from(body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);
    let created: Media = json(response).await;
    assert_eq!(created.title, "Custom Movie");
    assert_eq!(created.media_type, MediaType::Movie);

    let ext_ids = find_external_ids_by_media_ids(&app.db, std::slice::from_ref(&created.id))
        .await
        .unwrap();
    assert!(ext_ids.is_empty());

    let job = app.jobs.recv().await.unwrap();
    match job {
        Job::Sync(sync_job) => {
            assert_eq!(sync_job.media_id, Some(created.id));
            assert_eq!(sync_job.storefront, None);
            assert!(!sync_job.force);
        }
        _ => panic!("expected Job::Sync"),
    }
}

#[tokio::test]
async fn create_media_returns_validation_error_on_empty_title() {
    let app = test_app().await;

    let body = serde_json::json!({
        "title": "   ",
        "media_type": "game"
    });

    let response = app
        .router
        .oneshot(
            Request::builder()
                .uri("/media")
                .method("POST")
                .header("content-type", "application/json")
                .body(Body::from(body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
}
