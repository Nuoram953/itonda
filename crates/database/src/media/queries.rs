use sqlx::SqlitePool;

use uuid::Uuid;

use super::models::MediaRow;
use crate::{error::DatabaseError, media::MediaInsert};

pub async fn find_all(pool: &SqlitePool) -> Result<Vec<MediaRow>, DatabaseError> {
    sqlx::query_as!(
        MediaRow,
        r#"
    SELECT
        id,
        title,
        media_type,
        year
    FROM media
    "#
    )
    .fetch_all(pool)
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
            media_type,
            year
        )
        VALUES (?, ?, ?, ?)
        RETURNING
            id,
            title,
            media_type,
            year
        "#,
        id,
        media.title,
        media.media_type,
        media.year
    )
    .fetch_one(pool)
    .await
    .map_err(DatabaseError::from)
}
