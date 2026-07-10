use crate::{
    media::{MediaInsert, find_all, insert_media},
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
            year: Some(2001),
        },
    )
    .await
    .unwrap();

    assert!(!row.id.is_empty());
    assert_eq!(row.title, "Halo");
    assert_eq!(row.media_type, "game");
    assert_eq!(row.year, Some(2001));
}

#[tokio::test]
async fn insert_media_generates_unique_id() {
    let pool = setup_db().await;

    let first = insert_media(
        &pool,
        MediaInsert {
            title: "Halo".to_string(),
            media_type: "game".to_string(),
            year: Some(2001),
        },
    )
    .await
    .unwrap();

    let second = insert_media(
        &pool,
        MediaInsert {
            title: "Halo".to_string(),
            media_type: "game".to_string(),
            year: Some(2001),
        },
    )
    .await
    .unwrap();

    assert_ne!(first.id, second.id);
}

#[tokio::test]
async fn insert_media_allows_null_year() {
    let pool = setup_db().await;

    let row = insert_media(
        &pool,
        MediaInsert {
            title: "Halo".to_string(),
            media_type: "game".to_string(),
            year: None,
        },
    )
    .await
    .unwrap();

    assert!(!row.id.is_empty());
    assert_eq!(row.title, "Halo");
    assert_eq!(row.media_type, "game");
    assert_eq!(row.year, None);
}

#[tokio::test]
async fn find_all_returns_all_media() {
    let pool = setup_db().await;

    insert_media(
        &pool,
        MediaInsert {
            title: "Halo".to_string(),
            media_type: "game".to_string(),
            year: Some(2001),
        },
    )
    .await
    .unwrap();

    insert_media(
        &pool,
        MediaInsert {
            title: "The Matrix".to_string(),
            media_type: "movie".to_string(),
            year: Some(1999),
        },
    )
    .await
    .unwrap();

    let media = find_all(&pool).await.unwrap();

    assert_eq!(media.len(), 2);

    assert_eq!(media[0].title, "Halo");
    assert_eq!(media[0].media_type, "game");
    assert_eq!(media[0].year, Some(2001));

    assert_eq!(media[1].title, "The Matrix");
    assert_eq!(media[1].media_type, "movie");
    assert_eq!(media[1].year, Some(1999));
}

#[tokio::test]
async fn find_all_returns_empty_when_no_media_exists() {
    let pool = setup_db().await;

    let media = find_all(&pool).await.unwrap();

    assert_eq!(media.len(), 0);
}

#[tokio::test]
async fn find_all_returns_media_with_null_year() {
    let pool = setup_db().await;

    insert_media(
        &pool,
        MediaInsert {
            title: "Halo".to_string(),
            media_type: "game".to_string(),
            year: None,
        },
    )
    .await
    .unwrap();

    let media = find_all(&pool).await.unwrap();

    assert_eq!(media.len(), 1);

    assert_eq!(media[0].title, "Halo");
    assert_eq!(media[0].media_type, "game");
    assert_eq!(media[0].year, None);
}
