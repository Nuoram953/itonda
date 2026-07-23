use crate::{
    media::{
        MediaGameStorefrontUpsert, MediaInsert, find_all, find_media_by_title,
        find_media_game_storefront, insert_media, upsert_media_game_storefront,
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
async fn upsert_media_game_storefront_creates_relation() {
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

    let row = upsert_media_game_storefront(
        &pool,
        MediaGameStorefrontUpsert {
            media_id: media.id.clone(),
            storefront_id: 1,
            internal_id: "12345".to_string(),
        },
    )
    .await
    .unwrap();

    assert_eq!(row.media_id, media.id);
    assert_eq!(row.storefront_id, 1.to_string());
    assert_eq!(row.internal_id, "12345");
}

#[tokio::test]
async fn find_media_game_storefront_returns_relation() {
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

    upsert_media_game_storefront(
        &pool,
        MediaGameStorefrontUpsert {
            media_id: media.id.clone(),
            storefront_id: 1,
            internal_id: "12345".to_string(),
        },
    )
    .await
    .unwrap();

    let row = find_media_game_storefront(&pool, media.id, 1)
        .await
        .unwrap();

    assert!(row.is_some());

    let row = row.unwrap();

    assert_eq!(row.internal_id, "12345");
}

#[tokio::test]
async fn find_media_game_storefront_returns_none_when_missing() {
    let pool = setup_db().await;

    let row = find_media_game_storefront(&pool, "missing-id".to_string(), 1)
        .await
        .unwrap();

    assert!(row.is_none());
}

#[tokio::test]
async fn upsert_media_game_storefront_updates_existing_relation() {
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

    upsert_media_game_storefront(
        &pool,
        MediaGameStorefrontUpsert {
            media_id: media.id.clone(),
            storefront_id: 1,
            internal_id: "old-id".to_string(),
        },
    )
    .await
    .unwrap();

    let row = upsert_media_game_storefront(
        &pool,
        MediaGameStorefrontUpsert {
            media_id: media.id.clone(),
            storefront_id: 1,
            internal_id: "new-id".to_string(),
        },
    )
    .await
    .unwrap();

    assert_eq!(row.internal_id, "new-id");

    let stored = find_media_game_storefront(&pool, media.id, 1)
        .await
        .unwrap()
        .unwrap();

    assert_eq!(stored.internal_id, "new-id");
}
