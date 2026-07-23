use sqlx::SqlitePool;

use uuid::Uuid;

use super::models::MediaRow;
use crate::{
    error::DatabaseError,
    media::{MediaGameStorefrontRow, MediaGameStorefrontUpsert, MediaInsert},
};

pub async fn find_all(pool: &SqlitePool) -> Result<Vec<MediaRow>, DatabaseError> {
    sqlx::query_as!(
        MediaRow,
        r#"
    SELECT
        id,
        title,
        media_type
    FROM media
    "#
    )
    .fetch_all(pool)
    .await
    .map_err(DatabaseError::from)
}

pub async fn find_media_by_title(
    pool: &SqlitePool,
    title: String,
) -> Result<Option<MediaRow>, DatabaseError> {
    sqlx::query_as!(
        MediaRow,
        r#"
    SELECT
        id,
        title,
        media_type
    FROM media
    WHERE title=?
    "#,
        title
    )
    .fetch_optional(pool)
    .await
    .map_err(DatabaseError::from)
}

pub async fn insert_media(
    pool: &SqlitePool,
    media: MediaInsert,
) -> Result<MediaRow, DatabaseError> {
    let id = Uuid::new_v4().to_string();

    sqlx::query_as!(
        MediaRow,
        r#"
        INSERT INTO media (
            id,
            title,
            media_type
        )
        VALUES (?, ?, ?)
        RETURNING
            id,
            title,
            media_type
        "#,
        id,
        media.title,
        media.media_type,
    )
    .fetch_one(pool)
    .await
    .map_err(DatabaseError::from)
}

pub async fn upsert_media_game_storefront(
    pool: &SqlitePool,
    media_game_storefront: MediaGameStorefrontUpsert,
) -> Result<MediaGameStorefrontRow, DatabaseError> {
    sqlx::query_as!(
        MediaGameStorefrontRow,
        r#"
        INSERT INTO media_game_storefront (
            media_id,
            storefront_id,
            internal_id
        )
        VALUES (?, ?, ?)
        ON CONFLICT(media_id, storefront_id)
        DO UPDATE SET
            internal_id = excluded.internal_id
        RETURNING
            media_id,
            storefront_id,
            internal_id
        "#,
        media_game_storefront.media_id,
        media_game_storefront.storefront_id,
        media_game_storefront.internal_id,
    )
    .fetch_one(pool)
    .await
    .map_err(DatabaseError::from)
}

pub async fn find_media_game_storefront(
    pool: &SqlitePool,
    media_id: String,
    storefront_id: u32,
) -> Result<Option<MediaGameStorefrontRow>, DatabaseError> {
    sqlx::query_as!(
        MediaGameStorefrontRow,
        r#"
        SELECT
            media_id,
            storefront_id,
            internal_id
        FROM media_game_storefront
        WHERE media_id=? and storefront_id=?
        "#,
        media_id,
        storefront_id
    )
    .fetch_optional(pool)
    .await
    .map_err(DatabaseError::from)
}
