use itonda_database::{
    error::DatabaseError,
    media::{
        self as MediaQueries, MediaReviewUpsert, MediaThoughtInsert, MediaThoughtUpdate,
    },
};
use sqlx::SqlitePool;

use crate::reviews::{
    errors::ReviewError,
    models::{GameReview, GameReviewOverview, GameThought, ReviewVerdict},
};

async fn ensure_media_exists(pool: &SqlitePool, media_id: &str) -> Result<(), ReviewError> {
    match MediaQueries::find_media_by_id(pool, media_id.to_string()).await {
        Ok(_) => Ok(()),
        Err(DatabaseError::NotFound) => Err(ReviewError::MediaNotFound(media_id.to_string())),
        Err(err) => Err(ReviewError::Database(err)),
    }
}

pub async fn get_review_overview(
    pool: &SqlitePool,
    media_id: &str,
) -> Result<GameReviewOverview, ReviewError> {
    ensure_media_exists(pool, media_id).await?;

    let review_row = MediaQueries::find_review_by_media_id(pool, media_id).await?;
    let review = match review_row {
        Some(row) => Some(GameReview::try_from(row)?),
        None => None,
    };

    let thought_rows = MediaQueries::find_thoughts_by_media_id(pool, media_id).await?;
    let thoughts = thought_rows.into_iter().map(GameThought::from).collect();

    Ok(GameReviewOverview { review, thoughts })
}

pub async fn get_review(
    pool: &SqlitePool,
    media_id: &str,
) -> Result<Option<GameReview>, ReviewError> {
    ensure_media_exists(pool, media_id).await?;

    let review_row = MediaQueries::find_review_by_media_id(pool, media_id).await?;
    match review_row {
        Some(row) => Ok(Some(GameReview::try_from(row)?)),
        None => Ok(None),
    }
}

pub async fn upsert_review(
    pool: &SqlitePool,
    media_id: &str,
    verdict: ReviewVerdict,
    summary: Option<String>,
) -> Result<GameReview, ReviewError> {
    ensure_media_exists(pool, media_id).await?;

    let cleaned_summary = summary.and_then(|s| {
        let trimmed = s.trim();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed.to_string())
        }
    });

    let row = MediaQueries::upsert_review(
        pool,
        MediaReviewUpsert {
            media_id: media_id.to_string(),
            verdict: verdict.as_str().to_string(),
            summary: cleaned_summary,
        },
    )
    .await?;

    GameReview::try_from(row)
}

pub async fn delete_review(pool: &SqlitePool, media_id: &str) -> Result<(), ReviewError> {
    ensure_media_exists(pool, media_id).await?;

    MediaQueries::delete_review(pool, media_id).await?;
    Ok(())
}

pub async fn get_thoughts(
    pool: &SqlitePool,
    media_id: &str,
) -> Result<Vec<GameThought>, ReviewError> {
    ensure_media_exists(pool, media_id).await?;

    let rows = MediaQueries::find_thoughts_by_media_id(pool, media_id).await?;
    Ok(rows.into_iter().map(GameThought::from).collect())
}

pub async fn create_thought(
    pool: &SqlitePool,
    media_id: &str,
    title: String,
    content: String,
    category: Option<String>,
    playtime_minutes: Option<i64>,
) -> Result<GameThought, ReviewError> {
    ensure_media_exists(pool, media_id).await?;

    let trimmed_title = title.trim();
    if trimmed_title.is_empty() {
        return Err(ReviewError::Validation("Title cannot be empty".to_string()));
    }

    let trimmed_content = content.trim();
    if trimmed_content.is_empty() {
        return Err(ReviewError::Validation(
            "Content cannot be empty".to_string(),
        ));
    }

    let normalized_category = category
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .unwrap_or("general")
        .to_lowercase();

    let row = MediaQueries::insert_thought(
        pool,
        MediaThoughtInsert {
            media_id: media_id.to_string(),
            title: trimmed_title.to_string(),
            content: trimmed_content.to_string(),
            category: normalized_category,
            playtime_minutes,
        },
    )
    .await?;

    Ok(GameThought::from(row))
}

pub async fn update_thought(
    pool: &SqlitePool,
    thought_id: &str,
    title: String,
    content: String,
    category: Option<String>,
) -> Result<GameThought, ReviewError> {
    let trimmed_title = title.trim();
    if trimmed_title.is_empty() {
        return Err(ReviewError::Validation("Title cannot be empty".to_string()));
    }

    let trimmed_content = content.trim();
    if trimmed_content.is_empty() {
        return Err(ReviewError::Validation(
            "Content cannot be empty".to_string(),
        ));
    }

    let normalized_category = category
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .unwrap_or("general")
        .to_lowercase();

    let updated_row = MediaQueries::update_thought(
        pool,
        thought_id,
        MediaThoughtUpdate {
            title: trimmed_title.to_string(),
            content: trimmed_content.to_string(),
            category: normalized_category,
        },
    )
    .await?;

    match updated_row {
        Some(row) => Ok(GameThought::from(row)),
        None => Err(ReviewError::ThoughtNotFound(thought_id.to_string())),
    }
}

pub async fn delete_thought(pool: &SqlitePool, thought_id: &str) -> Result<(), ReviewError> {
    let deleted = MediaQueries::delete_thought(pool, thought_id).await?;
    if !deleted {
        return Err(ReviewError::ThoughtNotFound(thought_id.to_string()));
    }
    Ok(())
}
