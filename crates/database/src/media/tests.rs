use crate::{
    media::{
        MediaInsert, MediaLaunchUpsert, find_all, find_media_by_title, find_media_launch_by_id,
        find_media_launch_by_media_id, insert_media, upsert_media_launch,
    },
    test_utils::setup_db,
};

#[tokio::test]
async fn insert_media_creates_media() {
    let pool = setup_db().await;

    let row = insert_media(
        &pool,
        MediaInsert {
            title: "Halo".to_string(),
            media_type: "game".to_string(),
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

    let first = insert_media(
        &pool,
        MediaInsert {
            title: "Halo".to_string(),
            media_type: "game".to_string(),
        },
    )
    .await
    .unwrap();

    let second = insert_media(
        &pool,
        MediaInsert {
            title: "Halo".to_string(),
            media_type: "game".to_string(),
        },
    )
    .await
    .unwrap();

    assert_ne!(first.id, second.id);
}

#[tokio::test]
async fn find_all_returns_all_media() {
    let pool = setup_db().await;

    insert_media(
        &pool,
        MediaInsert {
            title: "Halo".to_string(),
            media_type: "game".to_string(),
        },
    )
    .await
    .unwrap();

    insert_media(
        &pool,
        MediaInsert {
            title: "The Matrix".to_string(),
            media_type: "movie".to_string(),
        },
    )
    .await
    .unwrap();

    let media = find_all(&pool).await.unwrap();

    assert_eq!(media.len(), 2);

    assert_eq!(media[0].title, "Halo");
    assert_eq!(media[0].media_type, "game");

    assert_eq!(media[1].title, "The Matrix");
    assert_eq!(media[1].media_type, "movie");
}

#[tokio::test]
async fn find_all_returns_empty_when_no_media_exists() {
    let pool = setup_db().await;

    let media = find_all(&pool).await.unwrap();

    assert_eq!(media.len(), 0);
}

#[tokio::test]
async fn find_media_by_title_returns_media_when_exists() {
    let pool = setup_db().await;

    insert_media(
        &pool,
        MediaInsert {
            title: "Halo".to_string(),
            media_type: "game".to_string(),
        },
    )
    .await
    .unwrap();

    let media = find_media_by_title(&pool, "Halo".to_string())
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

    let media = find_media_by_title(&pool, "Unknown".to_string())
        .await
        .unwrap();

    assert!(media.is_none());
}

#[tokio::test]
async fn upsert_media_launch_creates_launch() {
    let pool = setup_db().await;

    let media = insert_media(
        &pool,
        MediaInsert {
            title: "Halo".to_string(),
            media_type: "game".to_string(),
        },
    )
    .await
    .unwrap();

    let launch = upsert_media_launch(
        &pool,
        MediaLaunchUpsert {
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

    let media = insert_media(
        &pool,
        MediaInsert {
            title: "Halo".to_string(),
            media_type: "game".to_string(),
        },
    )
    .await
    .unwrap();

    upsert_media_launch(
        &pool,
        MediaLaunchUpsert {
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

    let launches = find_media_launch_by_media_id(&pool, media.id)
        .await
        .unwrap();

    assert_eq!(launches.len(), 1);
    assert_eq!(launches[0].name, "Default");
}

#[tokio::test]
async fn find_media_launch_by_media_id_returns_empty_when_missing() {
    let pool = setup_db().await;

    let launches = find_media_launch_by_media_id(&pool, "unknown".to_string())
        .await
        .unwrap();

    assert!(launches.is_empty());
}

#[tokio::test]
async fn find_media_launch_by_id_returns_launch() {
    let pool = setup_db().await;

    let media = insert_media(
        &pool,
        MediaInsert {
            title: "Halo".to_string(),
            media_type: "game".to_string(),
        },
    )
    .await
    .unwrap();

    let created = upsert_media_launch(
        &pool,
        MediaLaunchUpsert {
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

    let launch = find_media_launch_by_id(&pool, created.id).await.unwrap();

    assert_eq!(launch.name, "Default");
}

#[tokio::test]
async fn upsert_media_launch_updates_existing_launch() {
    let pool = setup_db().await;

    let media = insert_media(
        &pool,
        MediaInsert {
            title: "Halo".to_string(),
            media_type: "game".to_string(),
        },
    )
    .await
    .unwrap();

    let first = upsert_media_launch(
        &pool,
        MediaLaunchUpsert {
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

    let second = upsert_media_launch(
        &pool,
        MediaLaunchUpsert {
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

    assert_eq!(first.id, second.id);
    assert_eq!(second.arguments, r#"["new"]"#);
    assert!(second.is_default);
    assert!(!second.enabled);
}
