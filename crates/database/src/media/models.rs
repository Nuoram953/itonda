use serde::Serialize;
use sqlx::prelude::FromRow;

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct MediaRow {
    pub id: String,
    pub title: String,
    pub media_type: String,
    pub status_id: i64,
    pub description: Option<String>,
    pub summary: Option<String>,
    pub release_date: Option<i64>,
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

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct MediaStorefrontRow {
    pub media_id: String,
    pub storefront_id: String,
    pub external_id: String,
    pub playtime_minutes: Option<i64>,
    pub last_played_at: Option<i64>,
}

#[derive(Debug, Clone)]
pub struct MediaStorefrontUpsert {
    pub media_id: String,
    pub storefront_id: String,
    pub external_id: String,
    pub playtime_minutes: Option<i64>,
    pub last_played_at: Option<i64>,
}

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct MediaInstallationRow {
    pub id: String,
    pub media_id: String,
    pub agent_id: String,
    pub storefront_id: Option<String>,
    pub external_id: Option<String>,
    pub path: Option<String>,
}

#[derive(Debug, Clone)]
pub struct MediaInstallationUpsert {
    pub media_id: String,
    pub agent_id: String,
    pub storefront_id: Option<String>,
    pub external_id: Option<String>,
    pub path: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct MediaInsert {
    pub title: String,
    pub media_type: String,
    pub status_id: i64,
    pub description: Option<String>,
    pub summary: Option<String>,
    pub release_date: Option<i64>,
}

impl MediaInsert {
    pub fn new(title: impl Into<String>, media_type: impl Into<String>, status_id: i64) -> Self {
        Self {
            title: title.into(),
            media_type: media_type.into(),
            status_id,
            description: None,
            summary: None,
            release_date: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MediaMetadataUpdate {
    pub media_id: String,
    pub description: Option<String>,
    pub summary: Option<String>,
    pub release_date: Option<i64>,
}

#[derive(Debug, Clone)]
pub struct MediaAssetInsert {
    pub media_id: String,
    pub asset_id: i64,
    pub path: String,
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

#[derive(Debug, Clone, Default)]
pub struct MediaGameDetailsUpsert {
    pub media_id: String,
    pub playtime_minutes: Option<i64>,
    pub last_played_at: Option<i64>,
    pub series: Option<String>,
}

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct MediaGameDetailsRow {
    pub media_id: String,
    pub playtime_minutes: Option<i64>,
    pub last_played_at: Option<i64>,
    pub series: Option<String>,
}

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct GenreRow {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct TagRow {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct CompanyRoleRow {
    pub company_name: String,
    pub role_name: String,
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

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct MediaAssetSearchRow {
    pub media_id: String,
    pub asset_id: i64,
    pub searched_at: String,
}

#[derive(Debug, Clone)]
pub struct MediaAssetSearchInsert {
    pub media_id: String,
    pub asset_id: i64,
}

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct MediaMetadataSearchRow {
    pub media_id: String,
    pub searched_at: String,
}

#[derive(Debug, Clone)]
pub struct MediaMetadataSearchInsert {
    pub media_id: String,
}

#[derive(Debug, Clone, FromRow)]
pub struct MediaLaunchSessionRow {
    pub id: String,
    pub launch_id: String,
    pub started_at: String,
    pub completed_at: String,
    pub duration_seconds: String,
}

#[derive(Debug, Clone)]
pub struct MediaLaunchSessionInsert {
    pub launch_id: String,
    pub started_at: String,
    pub completed_at: String,
    pub duration_seconds: String,
}
