use serde::Serialize;
use sqlx::{prelude::FromRow, SqlitePool};
use uuid::Uuid;

use crate::error::DatabaseError;

#[derive(Debug, Clone, Serialize, FromRow, PartialEq, Eq)]
pub struct MediaReviewRow {
    pub media_id: String,
    pub verdict: String,
    pub summary: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone)]
pub struct MediaReviewUpsert {
    pub media_id: String,
    pub verdict: String,
    pub summary: Option<String>,
}

#[derive(Debug, Clone, Serialize, FromRow, PartialEq, Eq)]
pub struct MediaThoughtRow {
    pub id: String,
    pub media_id: String,
    pub title: String,
    pub content: String,
    pub category: String,
    pub playtime_minutes: Option<i64>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone)]
pub struct MediaThoughtInsert {
    pub media_id: String,
    pub title: String,
    pub content: String,
    pub category: String,
    pub playtime_minutes: Option<i64>,
}

#[derive(Debug, Clone)]
pub struct MediaThoughtUpdate {
    pub title: String,
    pub content: String,
    pub category: String,
}

pub async fn find_review_by_media_id(
    pool: &SqlitePool,
    media_id: &str,
) -> Result<Option<MediaReviewRow>, DatabaseError> {
    sqlx::query_as::<sqlx::Sqlite, MediaReviewRow>(
        "SELECT media_id, verdict, summary, created_at, updated_at FROM media_reviews WHERE media_id = $1",
    )
    .bind(media_id)
    .fetch_optional(pool)
    .await
    .map_err(DatabaseError::from)
}

pub async fn upsert_review(
    pool: &SqlitePool,
    review: MediaReviewUpsert,
) -> Result<MediaReviewRow, DatabaseError> {
    sqlx::query_as::<sqlx::Sqlite, MediaReviewRow>(
        r#"
        INSERT INTO media_reviews (media_id, verdict, summary, created_at, updated_at)
        VALUES ($1, $2, $3, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
        ON CONFLICT(media_id) DO UPDATE SET
            verdict = excluded.verdict,
            summary = excluded.summary,
            updated_at = CURRENT_TIMESTAMP
        RETURNING media_id, verdict, summary, created_at, updated_at
        "#,
    )
    .bind(&review.media_id)
    .bind(&review.verdict)
    .bind(&review.summary)
    .fetch_one(pool)
    .await
    .map_err(DatabaseError::from)
}

pub async fn delete_review(pool: &SqlitePool, media_id: &str) -> Result<bool, DatabaseError> {
    let result = sqlx::query("DELETE FROM media_reviews WHERE media_id = $1")
        .bind(media_id)
        .execute(pool)
        .await
        .map_err(DatabaseError::from)?;

    Ok(result.rows_affected() > 0)
}

pub async fn find_thoughts_by_media_id(
    pool: &SqlitePool,
    media_id: &str,
) -> Result<Vec<MediaThoughtRow>, DatabaseError> {
    sqlx::query_as::<sqlx::Sqlite, MediaThoughtRow>(
        r#"
        SELECT id, media_id, title, content, category, playtime_minutes, created_at, updated_at
        FROM media_thoughts
        WHERE media_id = $1
        ORDER BY created_at DESC
        "#,
    )
    .bind(media_id)
    .fetch_all(pool)
    .await
    .map_err(DatabaseError::from)
}

pub async fn find_thought_by_id(
    pool: &SqlitePool,
    id: &str,
) -> Result<Option<MediaThoughtRow>, DatabaseError> {
    sqlx::query_as::<sqlx::Sqlite, MediaThoughtRow>(
        r#"
        SELECT id, media_id, title, content, category, playtime_minutes, created_at, updated_at
        FROM media_thoughts
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(DatabaseError::from)
}

pub async fn insert_thought(
    pool: &SqlitePool,
    thought: MediaThoughtInsert,
) -> Result<MediaThoughtRow, DatabaseError> {
    let id = Uuid::new_v4().to_string();

    sqlx::query_as::<sqlx::Sqlite, MediaThoughtRow>(
        r#"
        INSERT INTO media_thoughts (id, media_id, title, content, category, playtime_minutes, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
        RETURNING id, media_id, title, content, category, playtime_minutes, created_at, updated_at
        "#,
    )
    .bind(&id)
    .bind(&thought.media_id)
    .bind(&thought.title)
    .bind(&thought.content)
    .bind(&thought.category)
    .bind(thought.playtime_minutes)
    .fetch_one(pool)
    .await
    .map_err(DatabaseError::from)
}

pub async fn update_thought(
    pool: &SqlitePool,
    id: &str,
    thought: MediaThoughtUpdate,
) -> Result<Option<MediaThoughtRow>, DatabaseError> {
    sqlx::query_as::<sqlx::Sqlite, MediaThoughtRow>(
        r#"
        UPDATE media_thoughts
        SET title = $1,
            content = $2,
            category = $3,
            updated_at = CURRENT_TIMESTAMP
        WHERE id = $4
        RETURNING id, media_id, title, content, category, playtime_minutes, created_at, updated_at
        "#,
    )
    .bind(&thought.title)
    .bind(&thought.content)
    .bind(&thought.category)
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(DatabaseError::from)
}

pub async fn delete_thought(pool: &SqlitePool, id: &str) -> Result<bool, DatabaseError> {
    let result = sqlx::query("DELETE FROM media_thoughts WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await
        .map_err(DatabaseError::from)?;

    Ok(result.rows_affected() > 0)
}
