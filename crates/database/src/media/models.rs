use serde::Serialize;
use sqlx::prelude::FromRow;

#[derive(Debug, Serialize, FromRow)]
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
    pub agent_id: Option<String>,
    pub name: String,
    pub launch_type: String,
    pub program: String,
    pub arguments: String,
    pub working_directory: Option<String>,
    pub is_default: bool,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct MediaLaunchUpsert {
    pub media_id: String,
    pub agent_id: Option<String>,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DbMediaSortField {
    Title,
    LastPlayedAt,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DbSortOrder {
    Asc,
    Desc,
}

#[derive(Debug, Clone, Default)]
pub struct DbMediaFilterOptions<'a> {
    pub media_type: Option<&'a str>,
    pub search: Option<&'a str>,
    pub status_id: Option<i64>,
    pub storefront_id: Option<&'a str>,
    pub sort_by: Option<DbMediaSortField>,
    pub sort_order: Option<DbSortOrder>,
    pub page: u32,
    pub limit: u32,
}

#[derive(Debug)]
pub struct PaginatedMediaRows {
    pub items: Vec<MediaRow>,
    pub total: u64,
}
