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
            ..Default::default()
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
            ..Default::default()
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
            ..Default::default()
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
            ..Default::default()
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
            ..Default::default()
        },
    )
    .await
    .unwrap();

    let media = MediaQueries::find_all(&pool, None).await.unwrap();

    assert_eq!(media.len(), 2);

    assert_eq!(media[0].title, "Halo");
    assert_eq!(media[0].media_type, "game");

    assert_eq!(media[1].title, "The Matrix");
    assert_eq!(media[1].media_type, "movie");
}

#[tokio::test]
async fn find_all_filters_by_media_type() {
    let pool = setup_db().await;

    MediaQueries::insert_media(
        &pool,
        MediaQueries::MediaInsert {
            title: "Halo".to_string(),
            media_type: "game".to_string(),
            status_id: 1,
            ..Default::default()
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
            ..Default::default()
        },
    )
    .await
    .unwrap();

    let games = MediaQueries::find_all(&pool, Some("game")).await.unwrap();

    assert_eq!(games.len(), 1);
    assert_eq!(games[0].title, "Halo");
    assert_eq!(games[0].media_type, "game");

    let movies = MediaQueries::find_all(&pool, Some("movie")).await.unwrap();

    assert_eq!(movies.len(), 1);
    assert_eq!(movies[0].title, "The Matrix");
    assert_eq!(movies[0].media_type, "movie");

    let tv_shows = MediaQueries::find_all(&pool, Some("tv_show"))
        .await
        .unwrap();

    assert_eq!(tv_shows.len(), 0);
}

#[tokio::test]
async fn find_all_returns_empty_when_no_media_exists() {
    let pool = setup_db().await;

    let media = MediaQueries::find_all(&pool, None).await.unwrap();

    assert_eq!(media.len(), 0);
}

#[tokio::test]
async fn find_paginated_supports_filters_sorting_and_pagination() {
    let pool = setup_db().await;

    let media1 = MediaQueries::insert_media(
        &pool,
        MediaQueries::MediaInsert {
            title: "Cyberpunk 2077".to_string(),
            media_type: "game".to_string(),
            status_id: 1,
            ..Default::default()
        },
    )
    .await
    .unwrap();

    let media2 = MediaQueries::insert_media(
        &pool,
        MediaQueries::MediaInsert {
            title: "Elden Ring".to_string(),
            media_type: "game".to_string(),
            status_id: 2,
            ..Default::default()
        },
    )
    .await
    .unwrap();

    let _media3 = MediaQueries::insert_media(
        &pool,
        MediaQueries::MediaInsert {
            title: "The Matrix".to_string(),
            media_type: "movie".to_string(),
            status_id: 1,
            ..Default::default()
        },
    )
    .await
    .unwrap();

    MediaQueries::upsert_media_game_details(
        &pool,
        MediaQueries::MediaGameDetailsUpsert {
            media_id: media1.id.clone(),
            playtime_minutes: Some(100),
            last_played_at: Some(1000),
            ..Default::default()
        },
    )
    .await
    .unwrap();

    MediaQueries::upsert_media_game_details(
        &pool,
        MediaQueries::MediaGameDetailsUpsert {
            media_id: media2.id.clone(),
            playtime_minutes: Some(500),
            last_played_at: Some(2000),
            ..Default::default()
        },
    )
    .await
    .unwrap();

    let result = MediaQueries::find_paginated(
        &pool,
        MediaQueries::DbMediaFilterOptions {
            media_type: Some("game"),
            sort_by: Some(MediaQueries::DbMediaSortField::Title),
            sort_order: Some(MediaQueries::DbSortOrder::Asc),
            page: 1,
            limit: 1,
            ..Default::default()
        },
    )
    .await
    .unwrap();

    assert_eq!(result.total, 2);
    assert_eq!(result.items.len(), 1);
    assert_eq!(result.items[0].title, "Cyberpunk 2077");

    let result_p2 = MediaQueries::find_paginated(
        &pool,
        MediaQueries::DbMediaFilterOptions {
            media_type: Some("game"),
            sort_by: Some(MediaQueries::DbMediaSortField::Title),
            sort_order: Some(MediaQueries::DbSortOrder::Asc),
            page: 2,
            limit: 1,
            ..Default::default()
        },
    )
    .await
    .unwrap();

    assert_eq!(result_p2.total, 2);
    assert_eq!(result_p2.items.len(), 1);
    assert_eq!(result_p2.items[0].title, "Elden Ring");

    let search_result = MediaQueries::find_paginated(
        &pool,
        MediaQueries::DbMediaFilterOptions {
            search: Some("elden"),
            ..Default::default()
        },
    )
    .await
    .unwrap();

    assert_eq!(search_result.total, 1);
    assert_eq!(search_result.items[0].title, "Elden Ring");

    let last_played_result = MediaQueries::find_paginated(
        &pool,
        MediaQueries::DbMediaFilterOptions {
            media_type: Some("game"),
            sort_by: Some(MediaQueries::DbMediaSortField::LastPlayedAt),
            sort_order: Some(MediaQueries::DbSortOrder::Desc),
            page: 1,
            limit: 10,
            ..Default::default()
        },
    )
    .await
    .unwrap();

    assert_eq!(last_played_result.items[0].title, "Elden Ring");
    assert_eq!(last_played_result.items[1].title, "Cyberpunk 2077");
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
            ..Default::default()
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
            ..Default::default()
        },
    )
    .await
    .unwrap();

    let launch = MediaQueries::upsert_media_launch(
        &pool,
        MediaQueries::MediaLaunchUpsert {
            media_id: media.id.clone(),
            agent_id: None,
            name: "Default".to_string(),
            launch_type: "steam".to_string(),
            program: "steam".to_string(),
            arguments: "[]".to_string(),
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
    assert!(launch.is_default);
    assert!(launch.enabled);
    assert_eq!(launch.arguments, "[]");
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
            ..Default::default()
        },
    )
    .await
    .unwrap();

    MediaQueries::upsert_media_launch(
        &pool,
        MediaQueries::MediaLaunchUpsert {
            media_id: media.id.clone(),
            agent_id: None,
            name: "Default".to_string(),
            launch_type: "steam".to_string(),
            program: "steam".to_string(),
            arguments: "[]".to_string(),
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
            ..Default::default()
        },
    )
    .await
    .unwrap();

    let launch = MediaQueries::upsert_media_launch(
        &pool,
        MediaQueries::MediaLaunchUpsert {
            media_id: media.id,
            agent_id: None,
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
            ..Default::default()
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
            ..Default::default()
        },
    )
    .await
    .unwrap();

    let launch = MediaQueries::MediaLaunchUpsert {
        media_id: media.id,
        agent_id: None,
        name: "Default".to_string(),
        launch_type: "steam".to_string(),
        program: "steam".to_string(),
        arguments: "[]".to_string(),
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
            ..Default::default()
        },
    )
    .await
    .unwrap();

    let first = MediaQueries::upsert_media_launch(
        &pool,
        MediaQueries::MediaLaunchUpsert {
            media_id: media.id.clone(),
            agent_id: None,
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
            agent_id: None,
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
            ..Default::default()
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

#[tokio::test]
async fn upsert_media_storefront_creates_and_updates() {
    let pool = setup_db().await;

    let media = MediaQueries::insert_media(
        &pool,
        MediaQueries::MediaInsert {
            title: "Portal 2".into(),
            media_type: "game".into(),
            status_id: 1,
            ..Default::default()
        },
    )
    .await
    .unwrap();

    let insert_res = MediaQueries::upsert_media_storefront(
        &pool,
        MediaQueries::MediaStorefrontUpsert {
            media_id: media.id.clone(),
            storefront_id: "0".into(),
            external_id: "620".into(),
            playtime_minutes: Some(60),
            last_played_at: Some(12345),
        },
    )
    .await
    .unwrap();

    assert_eq!(insert_res.action, UpsertAction::Created);
    assert_eq!(insert_res.value.external_id, "620");

    let unchanged_res = MediaQueries::upsert_media_storefront(
        &pool,
        MediaQueries::MediaStorefrontUpsert {
            media_id: media.id.clone(),
            storefront_id: "0".into(),
            external_id: "620".into(),
            playtime_minutes: Some(60),
            last_played_at: Some(12345),
        },
    )
    .await
    .unwrap();

    assert_eq!(unchanged_res.action, UpsertAction::Unchanged);

    let update_res = MediaQueries::upsert_media_storefront(
        &pool,
        MediaQueries::MediaStorefrontUpsert {
            media_id: media.id.clone(),
            storefront_id: "0".into(),
            external_id: "620_updated".into(),
            playtime_minutes: Some(120),
            last_played_at: Some(12346),
        },
    )
    .await
    .unwrap();

    assert_eq!(update_res.action, UpsertAction::Updated);
    assert_eq!(update_res.value.playtime_minutes, Some(120));
}

#[tokio::test]
async fn find_media_by_storefront_returns_correct_media() {
    let pool = setup_db().await;

    let media = MediaQueries::insert_media(
        &pool,
        MediaQueries::MediaInsert {
            title: "Half-Life 2".into(),
            media_type: "game".into(),
            status_id: 1,
            ..Default::default()
        },
    )
    .await
    .unwrap();

    MediaQueries::upsert_media_storefront(
        &pool,
        MediaQueries::MediaStorefrontUpsert {
            media_id: media.id.clone(),
            storefront_id: "0".into(),
            external_id: "220".into(),
            playtime_minutes: Some(120),
            last_played_at: Some(1723900000),
        },
    )
    .await
    .unwrap();

    let found = MediaQueries::find_media_by_storefront(&pool, "0", "220")
        .await
        .unwrap();
    assert!(found.is_some());
    assert_eq!(found.unwrap().id, media.id);

    let not_found = MediaQueries::find_media_by_storefront(&pool, "0", "999999")
        .await
        .unwrap();
    assert!(not_found.is_none());

    let storefronts = MediaQueries::find_storefronts_by_media_id(&pool, &media.id)
        .await
        .unwrap();
    assert_eq!(storefronts.len(), 1);
    assert_eq!(storefronts[0].storefront_id, "0");
    assert_eq!(storefronts[0].external_id, "220");
    assert_eq!(storefronts[0].playtime_minutes, Some(120));

    use crate::agent::{AgentsInsert, upsert_agent};
    upsert_agent(
        &pool,
        AgentsInsert {
            id: "agent-1".into(),
            name: "Test Agent".into(),
            hostname: "desktop".into(),
            platform: "linux".into(),
            agent_version: "1.0.0".into(),
        },
    )
    .await
    .unwrap();

    MediaQueries::upsert_media_installation(
        &pool,
        MediaQueries::MediaInstallationUpsert {
            media_id: media.id.clone(),
            agent_id: "agent-1".into(),
            storefront_id: Some("0".into()),
            external_id: Some("220".into()),
            path: Some("/games/hl2".into()),
        },
    )
    .await
    .unwrap();

    let installations = MediaQueries::find_installations_by_media_id(&pool, &media.id)
        .await
        .unwrap();
    assert_eq!(installations.len(), 1);
    assert_eq!(installations[0].agent_id, "agent-1");
    assert_eq!(installations[0].path, Some("/games/hl2".into()));
}

#[tokio::test]
async fn test_metadata_queries_crud() {
    let pool = setup_db().await;

    let media = MediaQueries::insert_media(
        &pool,
        MediaQueries::MediaInsert {
            title: "The Witcher 3".into(),
            media_type: "game".into(),
            status_id: 1,
            description: Some("Initial storyline".into()),
            summary: Some("Initial summary".into()),
            release_date: Some(1431993600),
        },
    )
    .await
    .unwrap();

    assert_eq!(media.description.as_deref(), Some("Initial storyline"));
    assert_eq!(media.summary.as_deref(), Some("Initial summary"));
    assert_eq!(media.release_date, Some(1431993600));

    MediaQueries::update_media_metadata(
        &pool,
        MediaQueries::MediaMetadataUpdate {
            media_id: media.id.clone(),
            description: Some("Updated storyline".into()),
            summary: Some("Updated summary".into()),
            release_date: Some(1431993600),
        },
    )
    .await
    .unwrap();

    let updated = MediaQueries::find_media_by_id(&pool, media.id.clone())
        .await
        .unwrap();
    assert_eq!(updated.description.as_deref(), Some("Updated storyline"));
    assert_eq!(updated.summary.as_deref(), Some("Updated summary"));

    MediaQueries::sync_media_genres(&pool, &media.id, &["RPG".into(), "Open World".into()])
        .await
        .unwrap();

    let genres = MediaQueries::find_genres_by_media_id(&pool, &media.id)
        .await
        .unwrap();
    assert_eq!(genres.len(), 2);
    assert!(genres.contains(&"RPG".to_string()));
    assert!(genres.contains(&"Open World".to_string()));

    MediaQueries::sync_media_tags(&pool, &media.id, &["Action".into(), "Fantasy".into()])
        .await
        .unwrap();

    let tags = MediaQueries::find_tags_by_media_id(&pool, &media.id)
        .await
        .unwrap();
    assert_eq!(tags.len(), 2);
    assert!(tags.contains(&"Action".to_string()));
    assert!(tags.contains(&"Fantasy".to_string()));

    MediaQueries::sync_media_companies(
        &pool,
        &media.id,
        &["CD Projekt RED".into()],
        &["CD Projekt".into()],
    )
    .await
    .unwrap();

    let companies = MediaQueries::find_companies_by_media_id(&pool, &media.id)
        .await
        .unwrap();
    assert_eq!(companies.len(), 2);

    let details = MediaQueries::upsert_media_game_details(
        &pool,
        MediaQueries::MediaGameDetailsUpsert {
            media_id: media.id.clone(),
            playtime_minutes: Some(150),
            last_played_at: Some(1700000000),
            series: Some("The Witcher".into()),
        },
    )
    .await
    .unwrap();

    assert_eq!(details.value.series.as_deref(), Some("The Witcher"));
}

#[tokio::test]
async fn test_metadata_searches_crud() {
    let pool = setup_db().await;

    let media = MediaQueries::insert_media(
        &pool,
        MediaQueries::MediaInsert {
            title: "Dark Souls".into(),
            media_type: "game".into(),
            status_id: 1,
            ..Default::default()
        },
    )
    .await
    .unwrap();

    let search_none = MediaQueries::find_metadata_search_by_media_id(&pool, &media.id)
        .await
        .unwrap();
    assert!(search_none.is_none());

    let inserted = MediaQueries::insert_media_metadata_search(
        &pool,
        MediaQueries::MediaMetadataSearchInsert {
            media_id: media.id.clone(),
        },
    )
    .await
    .unwrap();
    assert_eq!(inserted.media_id, media.id);

    let search_found = MediaQueries::find_metadata_search_by_media_id(&pool, &media.id)
        .await
        .unwrap();
    assert!(search_found.is_some());
    assert_eq!(search_found.unwrap().media_id, media.id);

    let batch = MediaQueries::find_metadata_searches_by_media_ids(&pool, &[media.id])
        .await
        .unwrap();
    assert_eq!(batch.len(), 1);
}
