use crate::media::{self as MediaQueries, MediaAssetInsert, insert_media_asset};
use crate::models::UpsertAction;
use crate::test_utils::setup_db;

#[tokio::test]
async fn insert_media_creates_media() {
    let pool = setup_db().await;

    let row = MediaQueries::insert_media(
        &pool,
        MediaQueries::MediaInsert {
            title: "Halo".to_string(),
            media_type: "game".to_string(),
            status_id: 1,
        },
    )
    .await
    .unwrap();

    assert!(!row.id.is_empty());
    assert_eq!(row.title, "Halo");
    assert_eq!(row.media_type, "game");
}

#[tokio::test]
async fn insert_media_generates_unique_id() {
    let pool = setup_db().await;

    let first = MediaQueries::insert_media(
        &pool,
        MediaQueries::MediaInsert {
            title: "Halo".to_string(),
            media_type: "game".to_string(),
            status_id: 1,
        },
    )
    .await
    .unwrap();

    let second = MediaQueries::insert_media(
        &pool,
        MediaQueries::MediaInsert {
            title: "Halo".to_string(),
            media_type: "game".to_string(),
            status_id: 1,
        },
    )
    .await
    .unwrap();

    assert_ne!(first.id, second.id);
}

#[tokio::test]
async fn find_all_returns_all_media() {
    let pool = setup_db().await;

    MediaQueries::insert_media(
        &pool,
        MediaQueries::MediaInsert {
            title: "Halo".to_string(),
            media_type: "game".to_string(),
            status_id: 1,
        },
    )
    .await
    .unwrap();

    MediaQueries::insert_media(
        &pool,
        MediaQueries::MediaInsert {
            title: "The Matrix".to_string(),
            media_type: "movie".to_string(),
            status_id: 1,
        },
    )
    .await
    .unwrap();

    let media = MediaQueries::find_all(&pool).await.unwrap();

    assert_eq!(media.len(), 2);

    assert_eq!(media[0].title, "Halo");
    assert_eq!(media[0].media_type, "game");

    assert_eq!(media[1].title, "The Matrix");
    assert_eq!(media[1].media_type, "movie");
}

#[tokio::test]
async fn find_all_returns_empty_when_no_media_exists() {
    let pool = setup_db().await;

    let media = MediaQueries::find_all(&pool).await.unwrap();

    assert_eq!(media.len(), 0);
}

#[tokio::test]
async fn find_media_by_title_returns_media_when_exists() {
    let pool = setup_db().await;

    MediaQueries::insert_media(
        &pool,
        MediaQueries::MediaInsert {
            title: "Halo".to_string(),
            media_type: "game".to_string(),
            status_id: 1,
        },
    )
    .await
    .unwrap();

    let media = MediaQueries::find_media_by_title(&pool, "Halo".to_string())
        .await
        .unwrap();

    assert!(media.is_some());

    let media = media.unwrap();

    assert_eq!(media.title, "Halo");
    assert_eq!(media.media_type, "game");
}

#[tokio::test]
async fn find_media_by_title_returns_none_when_missing() {
    let pool = setup_db().await;

    let media = MediaQueries::find_media_by_title(&pool, "Unknown".to_string())
        .await
        .unwrap();

    assert!(media.is_none());
}

#[tokio::test]
async fn upsert_media_launch_creates_launch() {
    let pool = setup_db().await;

    let media = MediaQueries::insert_media(
        &pool,
        MediaQueries::MediaInsert {
            title: "Halo".to_string(),
            media_type: "game".to_string(),
            status_id: 1,
        },
    )
    .await
    .unwrap();

    let launch = MediaQueries::upsert_media_launch(
        &pool,
        MediaQueries::MediaLaunchUpsert {
            media_id: media.id.clone(),
            name: "Default".to_string(),
            launch_type: "steam".to_string(),
            program: "steam".to_string(),
            arguments: r#"["steam://run/9310"]"#.to_string(),
            working_directory: None,
            is_default: true,
            enabled: true,
        },
    )
    .await
    .unwrap();

    let launch = launch.value;

    assert!(!launch.id.is_empty());
    assert_eq!(launch.media_id, media.id);
    assert_eq!(launch.name, "Default");
    assert_eq!(launch.launch_type, "steam");
    assert_eq!(launch.program, "steam");
    assert_eq!(launch.arguments, r#"["steam://run/9310"]"#);
    assert!(launch.is_default);
    assert!(launch.enabled);
}

#[tokio::test]
async fn find_media_launch_by_media_id_returns_launches() {
    let pool = setup_db().await;

    let media = MediaQueries::insert_media(
        &pool,
        MediaQueries::MediaInsert {
            title: "Halo".to_string(),
            media_type: "game".to_string(),
            status_id: 1,
        },
    )
    .await
    .unwrap();

    MediaQueries::upsert_media_launch(
        &pool,
        MediaQueries::MediaLaunchUpsert {
            media_id: media.id.clone(),
            name: "Default".to_string(),
            launch_type: "steam".to_string(),
            program: "steam".to_string(),
            arguments: r#"["steam://run/9310"]"#.to_string(),
            working_directory: None,
            is_default: true,
            enabled: true,
        },
    )
    .await
    .unwrap();

    let launches = MediaQueries::find_media_launch_by_media_id(&pool, media.id)
        .await
        .unwrap();

    assert_eq!(launches.len(), 1);
    assert_eq!(launches[0].name, "Default");
}

#[tokio::test]
async fn find_media_launch_by_media_id_returns_empty_when_missing() {
    let pool = setup_db().await;

    let launches = MediaQueries::find_media_launch_by_media_id(&pool, "unknown".to_string())
        .await
        .unwrap();

    assert!(launches.is_empty());
}

#[tokio::test]
async fn find_media_launch_by_id_returns_launch() {
    let pool = setup_db().await;

    let media = MediaQueries::insert_media(
        &pool,
        MediaQueries::MediaInsert {
            title: "Halo".to_string(),
            media_type: "game".to_string(),
            status_id: 1,
        },
    )
    .await
    .unwrap();

    let launch = MediaQueries::upsert_media_launch(
        &pool,
        MediaQueries::MediaLaunchUpsert {
            media_id: media.id,
            name: "Default".to_string(),
            launch_type: "steam".to_string(),
            program: "steam".to_string(),
            arguments: "[]".to_string(),
            working_directory: None,
            is_default: false,
            enabled: true,
        },
    )
    .await
    .unwrap();

    let launch = MediaQueries::find_media_launch_by_id(&pool, launch.value.id)
        .await
        .unwrap();

    assert_eq!(launch.name, "Default");
}

#[tokio::test]
async fn update_media_status_updates_media_and_creates_history() {
    let pool = setup_db().await;

    let media = MediaQueries::insert_media(
        &pool,
        MediaQueries::MediaInsert {
            title: "Halo".to_string(),
            media_type: "game".to_string(),
            status_id: 1,
        },
    )
    .await
    .unwrap();

    MediaQueries::update_media_status(&pool, &media.id, 2)
        .await
        .unwrap();

    let media = MediaQueries::find_media_by_title(&pool, media.title)
        .await
        .unwrap()
        .unwrap();

    assert_eq!(media.status_id, 2);

    let history = MediaQueries::find_media_status_history(&pool, &media.id)
        .await
        .unwrap();

    assert_eq!(history.len(), 1);
    assert_eq!(history[0].media_id, media.id);
    assert_eq!(history[0].status_id, 2);
}

#[tokio::test]
async fn upsert_media_launch_returns_unchanged_when_nothing_changed() {
    let pool = setup_db().await;

    let media = MediaQueries::insert_media(
        &pool,
        MediaQueries::MediaInsert {
            title: "Halo".to_string(),
            media_type: "game".to_string(),
            status_id: 1,
        },
    )
    .await
    .unwrap();

    let launch = MediaQueries::MediaLaunchUpsert {
        media_id: media.id,
        name: "Default".to_string(),
        launch_type: "steam".to_string(),
        program: "steam".to_string(),
        arguments: r#"["steam://run/9310"]"#.to_string(),
        working_directory: None,
        is_default: true,
        enabled: true,
    };

    let first = MediaQueries::upsert_media_launch(&pool, launch.clone())
        .await
        .unwrap();

    assert_eq!(first.action, UpsertAction::Created);

    let second = MediaQueries::upsert_media_launch(&pool, launch)
        .await
        .unwrap();

    assert_eq!(second.action, UpsertAction::Unchanged);
    assert_eq!(first.value.id, second.value.id);
}

#[tokio::test]
async fn upsert_media_launch_updates_existing_launch() {
    let pool = setup_db().await;

    let media = MediaQueries::insert_media(
        &pool,
        MediaQueries::MediaInsert {
            title: "Halo".to_string(),
            media_type: "game".to_string(),
            status_id: 1,
        },
    )
    .await
    .unwrap();

    let first = MediaQueries::upsert_media_launch(
        &pool,
        MediaQueries::MediaLaunchUpsert {
            media_id: media.id.clone(),
            name: "Default".to_string(),
            launch_type: "steam".to_string(),
            program: "steam".to_string(),
            arguments: r#"["old"]"#.to_string(),
            working_directory: None,
            is_default: false,
            enabled: true,
        },
    )
    .await
    .unwrap();

    assert_eq!(first.action, UpsertAction::Created);

    let second = MediaQueries::upsert_media_launch(
        &pool,
        MediaQueries::MediaLaunchUpsert {
            media_id: media.id,
            name: "Default".to_string(),
            launch_type: "steam".to_string(),
            program: "steam".to_string(),
            arguments: r#"["new"]"#.to_string(),
            working_directory: None,
            is_default: true,
            enabled: false,
        },
    )
    .await
    .unwrap();

    assert_eq!(second.action, UpsertAction::Updated);
    assert_eq!(first.value.id, second.value.id);
    assert_eq!(second.value.arguments, r#"["new"]"#);
    assert!(second.value.is_default);
    assert!(!second.value.enabled);
}

#[tokio::test]
async fn find_assets_by_media_ids_returns_assets() {
    let pool = setup_db().await;

    let media = MediaQueries::insert_media(
        &pool,
        MediaQueries::MediaInsert {
            title: "Halo".into(),
            media_type: "game".into(),
            status_id: 1,
        },
    )
    .await
    .unwrap();

    let _ = insert_media_asset(
        &pool,
        MediaAssetInsert {
            media_id: media.id.clone(),
            asset_id: 1,
            path: "path".into(),
        },
    )
    .await
    .unwrap();

    let assets = MediaQueries::find_assets_by_media_ids(&pool, &[media.id])
        .await
        .unwrap();

    assert_eq!(assets.len(), 1);
}
