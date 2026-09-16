use itonda_database::media::{MediaReviewRow, MediaThoughtRow};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::reviews::errors::ReviewError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum ReviewVerdict {
    Masterpiece,
    Recommended,
    Neutral,
    DoNotRecommend,
}

impl ReviewVerdict {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Masterpiece => "masterpiece",
            Self::Recommended => "recommended",
            Self::Neutral => "neutral",
            Self::DoNotRecommend => "do_not_recommend",
        }
    }
}

impl TryFrom<&str> for ReviewVerdict {
    type Error = ReviewError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "masterpiece" => Ok(Self::Masterpiece),
            "recommended" => Ok(Self::Recommended),
            "neutral" => Ok(Self::Neutral),
            "do_not_recommend" => Ok(Self::DoNotRecommend),
            _ => Err(ReviewError::InvalidVerdict(value.to_string())),
        }
    }
}

impl TryFrom<String> for ReviewVerdict {
    type Error = ReviewError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
pub struct GameReview {
    pub media_id: String,
    pub verdict: ReviewVerdict,
    pub summary: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl TryFrom<MediaReviewRow> for GameReview {
    type Error = ReviewError;

    fn try_from(row: MediaReviewRow) -> Result<Self, Self::Error> {
        Ok(Self {
            media_id: row.media_id,
            verdict: ReviewVerdict::try_from(row.verdict.as_str())?,
            summary: row.summary,
            created_at: row.created_at,
            updated_at: row.updated_at,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
pub struct GameThought {
    pub id: String,
    pub media_id: String,
    pub title: String,
    pub content: String,
    pub category: String,
    pub playtime_minutes: Option<i64>,
    pub created_at: String,
    pub updated_at: String,
}

impl From<MediaThoughtRow> for GameThought {
    fn from(row: MediaThoughtRow) -> Self {
        Self {
            id: row.id,
            media_id: row.media_id,
            title: row.title,
            content: row.content,
            category: row.category,
            playtime_minutes: row.playtime_minutes,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
pub struct GameReviewOverview {
    pub review: Option<GameReview>,
    pub thoughts: Vec<GameThought>,
}
