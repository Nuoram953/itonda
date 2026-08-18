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

#[tokio::test]
async fn returns_storefronts_and_installations() {
    use itonda_database::{
        agent::{AgentsInsert, upsert_agent},
        media::{
            MediaInstallationUpsert, MediaStorefrontUpsert, upsert_media_installation,
            upsert_media_storefront,
        },
    };
    use itonda_domain::storefronts::models::StorefrontId;

    let app = test_app().await;

    let media = MediaFixture {
        ..Default::default()
    };
    let fixture = media.insert(&app.db).await;

    // Upsert storefront
    upsert_media_storefront(
        &app.db,
        MediaStorefrontUpsert {
            media_id: fixture.media.id.clone(),
            storefront_id: "0".into(),
            external_id: "730".into(),
            playtime_minutes: Some(300),
            last_played_at: Some(1724000000),
        },
    )
    .await
    .unwrap();

    // Upsert agent and installation
    upsert_agent(
        &app.db,
        AgentsInsert {
            id: "agent-pc".into(),
            name: "Gaming PC".into(),
            hostname: "gaming-rig".into(),
            platform: "windows".into(),
            agent_version: "1.0.0".into(),
        },
    )
    .await
    .unwrap();

    upsert_media_installation(
        &app.db,
        MediaInstallationUpsert {
            media_id: fixture.media.id.clone(),
            agent_id: "agent-pc".into(),
            storefront_id: Some("0".into()),
            external_id: Some("730".into()),
            path: Some("C:\\Games\\Steam\\steamapps\\common\\CSGO".into()),
        },
    )
    .await
    .unwrap();

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
    assert_eq!(response.storefronts.len(), 1);
    assert_eq!(response.storefronts[0].storefront_id, StorefrontId::Steam);
    assert_eq!(response.storefronts[0].external_id, "730");
    assert_eq!(response.storefronts[0].playtime_minutes, Some(300));

    assert_eq!(response.installations.len(), 1);
    assert_eq!(response.installations[0].agent_id, "agent-pc");
    assert_eq!(
        response.installations[0].storefront_id,
        Some(StorefrontId::Steam)
    );
    assert_eq!(
        response.installations[0].path,
        Some("C:\\Games\\Steam\\steamapps\\common\\CSGO".into())
    );
}
