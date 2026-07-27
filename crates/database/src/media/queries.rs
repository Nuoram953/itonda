use sqlx::SqlitePool;

use uuid::Uuid;

use super::models::MediaRow;
use crate::{
    error::DatabaseError,
    media::{MediaInsert, MediaLaunchRow, MediaLaunchUpsert},
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

pub async fn find_media_launch_by_media_id(
    pool: &SqlitePool,
    media_id: String,
) -> Result<Vec<MediaLaunchRow>, DatabaseError> {
    sqlx::query_as!(
        MediaLaunchRow,
        r#"
        SELECT
            id,
            media_id,
            name,
            launch_type,
            program,
            arguments,
            working_directory,
            is_default AS "is_default: bool",
            enabled AS "enabled: bool"
        from media_launches
        where media_id=?
        "#,
        media_id,
    )
    .fetch_all(pool)
    .await
    .map_err(DatabaseError::from)
}

pub async fn find_media_launch_by_id(
    pool: &SqlitePool,
    launch_id: String,
) -> Result<MediaLaunchRow, DatabaseError> {
    sqlx::query_as!(
        MediaLaunchRow,
        r#"
        SELECT
            id,
            media_id,
            name,
            launch_type,
            program,
            arguments,
            working_directory,
            is_default AS "is_default: bool",
            enabled AS "enabled: bool"
        from media_launches
        where id=?
        "#,
        launch_id,
    )
    .fetch_one(pool)
    .await
    .map_err(DatabaseError::from)
}

pub async fn upsert_media_launch(
    pool: &SqlitePool,
    media_launch: MediaLaunchUpsert,
) -> Result<MediaLaunchRow, DatabaseError> {
    let id = Uuid::new_v4().to_string();
    sqlx::query_as!(
        MediaLaunchRow,
        r#"
        INSERT INTO media_launches (
            id,
            media_id,
            name,
            launch_type,
            program,
            arguments,
            working_directory,
            is_default,
            enabled
        )
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
        ON CONFLICT(media_id, name, launch_type)
        DO UPDATE SET
            program = excluded.program,
            arguments = excluded.arguments,
            working_directory = excluded.working_directory,
            is_default = excluded.is_default,
            enabled = excluded.enabled,
            updated_at = CURRENT_TIMESTAMP
        RETURNING
            id,
            media_id,
            name,
            launch_type,
            program,
            arguments,
            working_directory,
            is_default AS "is_default: bool",
            enabled AS "enabled: bool"
        "#,
        id,
        media_launch.media_id,
        media_launch.name,
        media_launch.launch_type,
        media_launch.program,
        media_launch.arguments,
        media_launch.working_directory,
        media_launch.is_default,
        media_launch.enabled,
    )
    .fetch_one(pool)
    .await
    .map_err(DatabaseError::from)
}
