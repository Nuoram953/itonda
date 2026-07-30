use sqlx::SqlitePool;

use uuid::Uuid;

use super::models::MediaRow;
use crate::{
    error::DatabaseError,
    media::{MediaInsert, MediaLaunchRow, MediaLaunchUpsert, MediaStatusHistoryRow},
    models::{UpsertAction, UpsertResult},
};

pub async fn find_all(pool: &SqlitePool) -> Result<Vec<MediaRow>, DatabaseError> {
    sqlx::query_as!(
        MediaRow,
        r#"
    SELECT
        id,
        title,
        media_type,
        status_id
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
        media_type,
        status_id
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
            media_type,
            status_id
        )
        VALUES (?, ?, ?, ?)
        RETURNING
            id,
            title,
            media_type,
            status_id
        "#,
        id,
        media.title,
        media.media_type,
        media.status_id
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
) -> Result<UpsertResult<MediaLaunchRow>, DatabaseError> {
    let existing = sqlx::query_as!(
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
        FROM media_launches
        WHERE media_id = ?
          AND name = ?
          AND launch_type = ?
        "#,
        media_launch.media_id,
        media_launch.name,
        media_launch.launch_type,
    )
    .fetch_optional(pool)
    .await
    .map_err(DatabaseError::from)?;

    let Some(existing) = existing else {
        let row = insert_media_launch(pool, media_launch).await?;

        return Ok(UpsertResult {
            value: row,
            action: UpsertAction::Created,
        });
    };

    let changed = existing.program != media_launch.program
        || existing.arguments != media_launch.arguments
        || existing.working_directory != media_launch.working_directory
        || existing.is_default != media_launch.is_default
        || existing.enabled != media_launch.enabled;

    if !changed {
        return Ok(UpsertResult {
            value: existing,
            action: UpsertAction::Unchanged,
        });
    }

    let row = sqlx::query_as!(
        MediaLaunchRow,
        r#"
        UPDATE media_launches
        SET
            program = ?,
            arguments = ?,
            working_directory = ?,
            is_default = ?,
            enabled = ?,
            updated_at = CURRENT_TIMESTAMP
        WHERE id = ?
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
        media_launch.program,
        media_launch.arguments,
        media_launch.working_directory,
        media_launch.is_default,
        media_launch.enabled,
        existing.id,
    )
    .fetch_one(pool)
    .await
    .map_err(DatabaseError::from)?;

    Ok(UpsertResult {
        value: row,
        action: UpsertAction::Updated,
    })
}

async fn insert_media_launch(
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

pub async fn update_media_status(
    pool: &SqlitePool,
    media_id: &str,
    status_id: i64,
) -> Result<(), DatabaseError> {
    let mut tx = pool.begin().await.map_err(DatabaseError::from)?;
    let id = Uuid::new_v4().to_string();

    sqlx::query!(
        r#"
        UPDATE media
        SET status_id = ?
        WHERE id = ?
        "#,
        status_id,
        media_id,
    )
    .execute(&mut *tx)
    .await
    .map_err(DatabaseError::from)?;

    sqlx::query!(
        r#"
        INSERT INTO media_status_history (
            id,
            media_id,
            status_id
        )
        VALUES (?, ?, ?)
        "#,
        id,
        media_id,
        status_id,
    )
    .execute(&mut *tx)
    .await
    .map_err(DatabaseError::from)?;

    tx.commit().await.map_err(DatabaseError::from)?;

    Ok(())
}

pub async fn find_media_status_history(
    pool: &SqlitePool,
    media_id: &str,
) -> Result<Vec<MediaStatusHistoryRow>, DatabaseError> {
    sqlx::query_as!(
        MediaStatusHistoryRow,
        r#"
        SELECT
            id,
            media_id,
            status_id,
            created_at
        FROM media_status_history
        WHERE media_id = ?
        ORDER BY created_at
        "#,
        media_id
    )
    .fetch_all(pool)
    .await
    .map_err(DatabaseError::from)
}
