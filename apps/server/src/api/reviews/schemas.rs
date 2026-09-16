use itonda_domain::reviews::models::ReviewVerdict;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpsertReviewPayload {
    pub verdict: ReviewVerdict,
    pub summary: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateThoughtPayload {
    pub title: String,
    pub content: String,
    pub category: Option<String>,
    pub playtime_minutes: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateThoughtPayload {
    pub title: String,
    pub content: String,
    pub category: Option<String>,
}
