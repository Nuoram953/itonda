use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use itonda_domain::reviews::models::{
    GameReview, GameReviewOverview, GameThought, ReviewVerdict,
};
use itonda_server::api::error::{ApiError, ErrorResponse};
use serde_json::json as json_value;
use tower::ServiceExt;
use uuid::Uuid;

use crate::common::{app::test_app, fixtures::media::MediaFixture, response::json};

#[tokio::test]
async fn get_review_overview_returns_empty_when_no_review_or_thoughts() {
    let app = test_app().await;
    let fixture = MediaFixture::default().insert(&app.db).await;

    let response = app
        .router
        .oneshot(
            Request::builder()
                .uri(format!("/media/{}/review", fixture.media.id))
                .method("GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let overview: GameReviewOverview = json(response).await;
    assert!(overview.review.is_none());
    assert!(overview.thoughts.is_empty());
}

#[tokio::test]
async fn upsert_and_delete_game_review() {
    let app = test_app().await;
    let fixture = MediaFixture::default().insert(&app.db).await;

    // 1. Create review
    let create_payload = json_value!({
        "verdict": "masterpiece",
        "summary": "Incredible campaign and worldbuilding."
    });

    let response = app
        .router
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!("/media/{}/review", fixture.media.id))
                .method("PUT")
                .header("content-type", "application/json")
                .body(Body::from(create_payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let review: GameReview = json(response).await;
    assert_eq!(review.media_id, fixture.media.id);
    assert_eq!(review.verdict, ReviewVerdict::Masterpiece);
    assert_eq!(
        review.summary,
        Some("Incredible campaign and worldbuilding.".into())
    );

    // 2. Update review
    let update_payload = json_value!({
        "verdict": "recommended",
        "summary": "Great game overall with fun co-op."
    });

    let response = app
        .router
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!("/media/{}/review", fixture.media.id))
                .method("PUT")
                .header("content-type", "application/json")
                .body(Body::from(update_payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let updated: GameReview = json(response).await;
    assert_eq!(updated.verdict, ReviewVerdict::Recommended);
    assert_eq!(
        updated.summary,
        Some("Great game overall with fun co-op.".into())
    );

    // 3. Verify in get_review_overview
    let response = app
        .router
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!("/media/{}/review", fixture.media.id))
                .method("GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let overview: GameReviewOverview = json(response).await;
    assert!(overview.review.is_some());
    assert_eq!(overview.review.unwrap().verdict, ReviewVerdict::Recommended);

    // 4. Delete review
    let response = app
        .router
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!("/media/{}/review", fixture.media.id))
                .method("DELETE")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    // 5. Verify review is gone
    let response = app
        .router
        .oneshot(
            Request::builder()
                .uri(format!("/media/{}/review", fixture.media.id))
                .method("GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let overview: GameReviewOverview = json(response).await;
    assert!(overview.review.is_none());
}

#[tokio::test]
async fn thoughts_crud_lifecycle() {
    let app = test_app().await;
    let fixture = MediaFixture::default().insert(&app.db).await;

    // 1. Create thought
    let create_payload = json_value!({
        "title": "Soundtrack in Gears",
        "content": "The music of gears really surprised me during the combat section.",
        "category": "audio",
        "playtime_minutes": 150
    });

    let response = app
        .router
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!("/media/{}/thoughts", fixture.media.id))
                .method("POST")
                .header("content-type", "application/json")
                .body(Body::from(create_payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);
    let thought: GameThought = json(response).await;
    assert_eq!(thought.title, "Soundtrack in Gears");
    assert_eq!(thought.category, "audio");
    assert_eq!(thought.playtime_minutes, Some(150));

    // 2. Get thoughts
    let response = app
        .router
        .clone()
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
    assert_eq!(thoughts.len(), 1);
    assert_eq!(thoughts[0].id, thought.id);

    // 3. Update thought
    let update_payload = json_value!({
        "title": "Orchestral Score",
        "content": "Updated thoughts on the incredible soundtrack.",
        "category": "audio"
    });

    let response = app
        .router
        .clone()
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
    assert_eq!(updated.title, "Orchestral Score");
    assert_eq!(
        updated.content,
        "Updated thoughts on the incredible soundtrack."
    );

    // 4. Delete thought
    let response = app
        .router
        .clone()
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

    // 5. Verify list is now empty
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
    let remaining: Vec<GameThought> = json(response).await;
    assert!(remaining.is_empty());
}

#[tokio::test]
async fn returns_404_when_media_not_found() {
    let app = test_app().await;
    let missing_media_id = Uuid::new_v4().to_string();

    let response = app
        .router
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!("/media/{missing_media_id}/review"))
                .method("GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    let error: ErrorResponse = json(response).await;
    assert_eq!(error.code, ApiError::MediaNotFound.error_body().code);
}

#[tokio::test]
async fn returns_422_when_thought_validation_fails() {
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
