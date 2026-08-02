use serde::Serialize;
use sqlx::prelude::FromRow;

#[derive(Debug, Serialize)]
pub struct MediaRow {
    pub id: String,
    pub title: String,
    pub media_type: String,
    pub status_id: i64,
}

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct MediaAssetRow {
    pub id: String,
    pub media_id: String,
    pub asset_id: i64,
    pub path: String,
}

#[derive(Debug, Serialize)]
pub struct MediaStatusHistoryRow {
    pub id: String,
    pub media_id: String,
    pub status_id: i64,
    pub created_at: String,
}

#[derive(Debug, Serialize)]
pub struct MediaGameStorefrontRow {
    pub media_id: String,
    pub storefront_id: String,
    pub internal_id: String,
}

#[derive(Debug)]
pub struct MediaInsert {
    pub title: String,
    pub media_type: String,
    pub status_id: i64,
}

#[derive(Debug, Clone)]
pub struct MediaAssetInsert {
    pub media_id: String,
    pub asset_id: i64,
    pub path: String,
}

#[derive(Debug)]
pub struct MediaGameStorefrontUpsert {
    pub media_id: String,
    pub storefront_id: u32,
    pub internal_id: String,
}

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct MediaLaunchRow {
    pub id: String,
    pub media_id: String,
    pub name: String,
    pub launch_type: String,
    pub program: String,
    pub arguments: String,
    pub working_directory: Option<String>,
    pub is_default: i64,
    pub enabled: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct MediaLaunchUpsert {
    pub media_id: String,
    pub name: String,
    pub launch_type: String,
    pub program: String,
    pub arguments: String,
    pub working_directory: Option<String>,
    pub is_default: bool,
    pub enabled: bool,
}

pub struct MediaGameDetailsUpsert {
    pub media_id: String,
    pub playtime_minutes: Option<i64>,
    pub last_played_at: Option<i64>,
}

pub struct MediaGameDetailsRow {
    pub media_id: String,
    pub playtime_minutes: Option<i64>,
    pub last_played_at: Option<i64>,
}
