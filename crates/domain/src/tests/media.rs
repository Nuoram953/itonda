use itonda_database::{
    media::{
        self as MediaQueries, MediaInsert, MediaLaunchSessionInsert, MediaLaunchUpsert,
        MediaStorefrontUpsert, insert_media, upsert_media_launch, upsert_media_storefront,
    },
    test_utils::setup_db,
};

use crate::{
    media::{
        service::{recalculate_media_game_details, update_playtime},
        types::MediaStatus,
    },
    storefronts::models::StorefrontId,
};

#[tokio::test]
async fn test_update_playtime_creates_game_details() {
    let pool = setup_db().await;

    let media = insert_media(
        &pool,
        MediaInsert {
            title: "Half-Life 2".into(),
            media_type: "game".into(),
            status_id: MediaStatus::NotStarted.id(),
            ..Default::default()
        },
    )
    .await
    .unwrap();

    let launch = upsert_media_launch(
        &pool,
        MediaLaunchUpsert {
            media_id: media.id.clone(),
            agent_id: None,
            name: "Default".into(),
            launch_type: "steam".into(),
            program: "hl2.exe".into(),
            arguments: "".into(),
            working_directory: None,
            is_default: true,
            enabled: true,
        },
    )
    .await
    .unwrap();

    let session = MediaLaunchSessionInsert {
        launch_id: launch.value.id.clone(),
        started_at: "2026-08-23 10:00:00 UTC".into(),
        completed_at: "2026-08-23 11:30:00 UTC".into(),
        duration_seconds: "5400".into(), // 90 minutes
    };

    update_playtime(&pool, session).await.unwrap();

    let details = MediaQueries::find_game_details(&pool, &media.id)
        .await
        .unwrap()
        .expect("game details should exist");

    assert_eq!(details.playtime_minutes, Some(90));
    assert!(details.last_played_at.is_some());
}

#[tokio::test]
async fn test_update_playtime_appends_to_existing_playtime() {
    let pool = setup_db().await;

    let media = insert_media(
        &pool,
        MediaInsert {
            title: "Portal".into(),
            media_type: "game".into(),
            status_id: MediaStatus::NotStarted.id(),
            ..Default::default()
        },
    )
    .await
    .unwrap();

    upsert_media_storefront(
        &pool,
        MediaStorefrontUpsert {
            media_id: media.id.clone(),
            storefront_id: StorefrontId::Steam.as_str().into(),
            external_id: "400".into(),
            playtime_minutes: Some(60),
            last_played_at: Some(1000),
        },
    )
    .await
    .unwrap();

    recalculate_media_game_details(&pool, &media.id)
        .await
        .unwrap();

    let launch = upsert_media_launch(
        &pool,
        MediaLaunchUpsert {
            media_id: media.id.clone(),
            agent_id: None,
            name: "Steam".into(),
            launch_type: "storefront".into(),
            program: "portal.exe".into(),
            arguments: "".into(),
            working_directory: None,
            is_default: true,
            enabled: true,
        },
    )
    .await
    .unwrap();

    let session = MediaLaunchSessionInsert {
        launch_id: launch.value.id.clone(),
        started_at: "2026-08-23 12:00:00 UTC".into(),
        completed_at: "2026-08-23 12:45:00 UTC".into(),
        duration_seconds: "2700".into(), // 45 minutes
    };

    update_playtime(&pool, session).await.unwrap();

    let details = MediaQueries::find_game_details(&pool, &media.id)
        .await
        .unwrap()
        .expect("game details should exist");

    assert_eq!(details.playtime_minutes, Some(105)); // 60 + 45
    assert_ne!(details.last_played_at, Some(1000));

    let storefronts = MediaQueries::find_storefronts_by_media_id(&pool, &media.id)
        .await
        .unwrap();
    assert_eq!(storefronts[0].playtime_minutes, Some(105));
}

#[tokio::test]
async fn test_update_playtime_storefront_launch_does_not_double_count_after_sync() {
    let pool = setup_db().await;

    let media = insert_media(
        &pool,
        MediaInsert {
            title: "Cyberpunk 2077".into(),
            media_type: "game".into(),
            status_id: MediaStatus::NotStarted.id(),
            ..Default::default()
        },
    )
    .await
    .unwrap();

    upsert_media_storefront(
        &pool,
        MediaStorefrontUpsert {
            media_id: media.id.clone(),
            storefront_id: StorefrontId::Steam.as_str().into(),
            external_id: "1091500".into(),
            playtime_minutes: Some(60),
            last_played_at: Some(1000),
        },
    )
    .await
    .unwrap();

    recalculate_media_game_details(&pool, &media.id)
        .await
        .unwrap();

    let launch = upsert_media_launch(
        &pool,
        MediaLaunchUpsert {
            media_id: media.id.clone(),
            agent_id: None,
            name: "Steam".into(),
            launch_type: "storefront".into(),
            program: "steam".into(),
            arguments: "".into(),
            working_directory: None,
            is_default: true,
            enabled: true,
        },
    )
    .await
    .unwrap();

    // 30 minute session played through storefront launch
    let session = MediaLaunchSessionInsert {
        launch_id: launch.value.id.clone(),
        started_at: "2026-08-23 12:00:00 UTC".into(),
        completed_at: "2026-08-23 12:30:00 UTC".into(),
        duration_seconds: "1800".into(), // 30 minutes
    };

    update_playtime(&pool, session).await.unwrap();

    let details = MediaQueries::find_game_details(&pool, &media.id)
        .await
        .unwrap()
        .expect("game details should exist");
    assert_eq!(details.playtime_minutes, Some(90)); // 60 + 30

    // Simulate later Steam sync reporting the new total playtime (90 min)
    upsert_media_storefront(
        &pool,
        MediaStorefrontUpsert {
            media_id: media.id.clone(),
            storefront_id: StorefrontId::Steam.as_str().into(),
            external_id: "1091500".into(),
            playtime_minutes: Some(90),
            last_played_at: Some(2000),
        },
    )
    .await
    .unwrap();

    recalculate_media_game_details(&pool, &media.id)
        .await
        .unwrap();

    let details_after_sync = MediaQueries::find_game_details(&pool, &media.id)
        .await
        .unwrap()
        .expect("game details should exist");

    // Must still be 90, NOT 120 (no double counting)
    assert_eq!(details_after_sync.playtime_minutes, Some(90));
}

#[tokio::test]
async fn test_update_playtime_custom_launch_preserved_after_storefront_sync() {
    let pool = setup_db().await;

    let media = insert_media(
        &pool,
        MediaInsert {
            title: "Super Mario 64".into(),
            media_type: "game".into(),
            status_id: MediaStatus::NotStarted.id(),
            ..Default::default()
        },
    )
    .await
    .unwrap();

    let launch = upsert_media_launch(
        &pool,
        MediaLaunchUpsert {
            media_id: media.id.clone(),
            agent_id: None,
            name: "RetroArch".into(),
            launch_type: "emulator".into(),
            program: "retroarch".into(),
            arguments: "".into(),
            working_directory: None,
            is_default: true,
            enabled: true,
        },
    )
    .await
    .unwrap();

    let session = MediaLaunchSessionInsert {
        launch_id: launch.value.id.clone(),
        started_at: "2026-08-23 14:00:00 UTC".into(),
        completed_at: "2026-08-23 14:45:00 UTC".into(),
        duration_seconds: "2700".into(), // 45 minutes
    };

    update_playtime(&pool, session).await.unwrap();

    let details = MediaQueries::find_game_details(&pool, &media.id)
        .await
        .unwrap()
        .expect("game details should exist");
    assert_eq!(details.playtime_minutes, Some(45));

    // Another custom session (15 minutes)
    let session2 = MediaLaunchSessionInsert {
        launch_id: launch.value.id.clone(),
        started_at: "2026-08-23 15:00:00 UTC".into(),
        completed_at: "2026-08-23 15:15:00 UTC".into(),
        duration_seconds: "900".into(), // 15 minutes
    };

    update_playtime(&pool, session2).await.unwrap();

    let details2 = MediaQueries::find_game_details(&pool, &media.id)
        .await
        .unwrap()
        .expect("game details should exist");
    assert_eq!(details2.playtime_minutes, Some(60));
}

#[tokio::test]
async fn test_multiple_storefronts_playtime_aggregated() {
    let pool = setup_db().await;

    sqlx::query("INSERT INTO storefronts (id, name) VALUES (?, ?)")
        .bind("1")
        .bind("GOG")
        .execute(&pool)
        .await
        .unwrap();

    let media = insert_media(
        &pool,
        MediaInsert {
            title: "Witcher 3".into(),
            media_type: "game".into(),
            status_id: MediaStatus::NotStarted.id(),
            ..Default::default()
        },
    )
    .await
    .unwrap();

    // Storefront 1: Steam (50 min)
    upsert_media_storefront(
        &pool,
        MediaStorefrontUpsert {
            media_id: media.id.clone(),
            storefront_id: StorefrontId::Steam.as_str().into(),
            external_id: "292030".into(),
            playtime_minutes: Some(50),
            last_played_at: Some(1000),
        },
    )
    .await
    .unwrap();

    // Storefront 2: GOG (30 min)
    upsert_media_storefront(
        &pool,
        MediaStorefrontUpsert {
            media_id: media.id.clone(),
            storefront_id: "1".into(),
            external_id: "gog-123".into(),
            playtime_minutes: Some(30),
            last_played_at: Some(2000),
        },
    )
    .await
    .unwrap();

    recalculate_media_game_details(&pool, &media.id)
        .await
        .unwrap();

    let details = MediaQueries::find_game_details(&pool, &media.id)
        .await
        .unwrap()
        .expect("game details should exist");
    assert_eq!(details.playtime_minutes, Some(80)); // 50 + 30
    assert_eq!(details.last_played_at, Some(2000));

    // Steam launch
    let steam_launch = upsert_media_launch(
        &pool,
        MediaLaunchUpsert {
            media_id: media.id.clone(),
            agent_id: None,
            name: "Steam".into(),
            launch_type: "storefront".into(),
            program: "steam".into(),
            arguments: "".into(),
            working_directory: None,
            is_default: true,
            enabled: true,
        },
    )
    .await
    .unwrap();

    // Play on Steam for 20 min
    let session = MediaLaunchSessionInsert {
        launch_id: steam_launch.value.id.clone(),
        started_at: "2026-08-23 16:00:00 UTC".into(),
        completed_at: "2026-08-23 16:20:00 UTC".into(),
        duration_seconds: "1200".into(),
    };

    update_playtime(&pool, session).await.unwrap();

    let details_after_play = MediaQueries::find_game_details(&pool, &media.id)
        .await
        .unwrap()
        .expect("game details should exist");
    assert_eq!(details_after_play.playtime_minutes, Some(100)); // 70 + 30

    // Custom launch
    let custom_launch = upsert_media_launch(
        &pool,
        MediaLaunchUpsert {
            media_id: media.id.clone(),
            agent_id: None,
            name: "Modded".into(),
            launch_type: "custom".into(),
            program: "witcher3_mod.exe".into(),
            arguments: "".into(),
            working_directory: None,
            is_default: false,
            enabled: true,
        },
    )
    .await
    .unwrap();

    // Play modded for 15 min
    let custom_session = MediaLaunchSessionInsert {
        launch_id: custom_launch.value.id.clone(),
        started_at: "2026-08-23 17:00:00 UTC".into(),
        completed_at: "2026-08-23 17:15:00 UTC".into(),
        duration_seconds: "900".into(),
    };

    update_playtime(&pool, custom_session).await.unwrap();

    let details_after_mod = MediaQueries::find_game_details(&pool, &media.id)
        .await
        .unwrap()
        .expect("game details should exist");
    assert_eq!(details_after_mod.playtime_minutes, Some(115)); // 70 (Steam) + 30 (GOG) + 15 (custom)
}
