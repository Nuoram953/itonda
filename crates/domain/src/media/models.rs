use itonda_database::media::{MediaAssetRow, MediaGameDetailsRow, MediaLaunchRow, MediaRow};
use serde::{Deserialize, Serialize};

use utoipa::ToSchema;

use crate::{
    assets::{error::AssetError, types::AssetType},
    media::{
        errors::MediaError,
        types::{MediaStatus, MediaType},
    },
};

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Media {
    pub id: String,
    pub title: String,
    pub media_type: MediaType,
    pub status: MediaStatus,
    pub assets: Vec<Asset>,
    pub details: Option<MediaDetails>,
    pub launches: Vec<Launch>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PaginatedMedia {
    pub items: Vec<Media>,
    pub total: u64,
    pub page: u32,
    pub limit: u32,
    pub total_pages: u32,
    pub has_next: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Asset {
    pub id: String,
    pub asset_type: AssetType,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Launch {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(untagged)]
pub enum MediaDetails {
    Game(MediaGameDetails),
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct MediaGameDetails {
    pub playtime_minutes: Option<i64>,
    pub last_played_at: Option<i64>,
}

impl TryFrom<MediaAssetRow> for Asset {
    type Error = AssetError;

    fn try_from(row: MediaAssetRow) -> Result<Self, Self::Error> {
        Ok(Self {
            id: row.id,
            asset_type: AssetType::try_from(row.asset_id)?,
        })
    }
}

impl From<MediaLaunchRow> for Launch {
    fn from(row: MediaLaunchRow) -> Self {
        Self {
            id: row.id,
            name: row.name,
        }
    }
}
impl TryFrom<MediaRow> for Media {
    type Error = MediaError;

    fn try_from(row: MediaRow) -> Result<Self, Self::Error> {
        Ok(Self {
            id: row.id,
            title: row.title,
            status: row.status_id.try_into()?,
            media_type: row.media_type.try_into()?,
            assets: Vec::new(),
            launches: Vec::new(),
            details: None,
        })
    }
}

impl From<MediaGameDetailsRow> for MediaGameDetails {
    fn from(row: MediaGameDetailsRow) -> Self {
        Self {
            playtime_minutes: row.playtime_minutes,
            last_played_at: row.last_played_at,
        }
    }
}
