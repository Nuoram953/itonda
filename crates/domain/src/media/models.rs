use itonda_database::media::{MediaAssetRow, MediaRow};
use serde::{Deserialize, Serialize};

use utoipa::ToSchema;

use crate::{media::errors::MediaError, storefronts::models::StorefrontId};

#[derive(Clone)]
pub struct DiscoveredMedia {
    pub storefront: StorefrontId,
    pub external_id: String,
    pub media_type: MediaType,
    pub title: String,
    pub metadata: DiscoveredMediaMetadata,
    pub launch: Option<DiscoveredLaunch>,
}

#[derive(Clone)]
pub struct DiscoveredLaunch {
    pub name: String,
    pub launch_type: MediaLaunchType,
    pub program: String,
    pub arguments: Vec<String>,
    pub working_directory: Option<String>,
}

#[derive(Clone)]
pub struct DiscoveredMediaMetadata {
    pub total_playtime: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Media {
    pub id: String,
    pub title: String,
    pub media_type: String,
    pub assets: Vec<Asset>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Asset {
    pub url: String,
    pub asset_type: AssetType,
}

impl TryFrom<MediaAssetRow> for Asset {
    type Error = MediaError;

    fn try_from(row: MediaAssetRow) -> Result<Self, Self::Error> {
        Ok(Self {
            asset_type: AssetType::try_from(row.asset_id)?,
            url: row.path,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum MediaType {
    Game,
    Movie,
    TvShow,
}

impl MediaType {
    pub fn as_str(&self) -> &'static str {
        match self {
            MediaType::Game => "game",
            MediaType::Movie => "movie",
            MediaType::TvShow => "tv_show",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MediaLaunchType {
    Storefront,
    Emulator,
    Custom,
}

impl MediaLaunchType {
    pub fn as_str(&self) -> &'static str {
        match self {
            MediaLaunchType::Storefront => "storefront",
            MediaLaunchType::Emulator => "emulator",
            MediaLaunchType::Custom => "custom",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum MediaSource {
    Steam,
}

impl TryFrom<MediaRow> for Media {
    type Error = MediaError;

    fn try_from(row: MediaRow) -> Result<Self, Self::Error> {
        Ok(Self {
            id: row.id,
            title: row.title,
            media_type: row
                .media_type
                .parse()
                .map_err(|_| MediaError::InvalidMediaType)?,
            assets: Vec::new(),
        })
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema)]
#[repr(i64)]
pub enum MediaStatus {
    NotStarted = 1,
    InProgress = 2,
    Completed = 3,
    Abandoned = 4,
    Paused = 5,
}

impl MediaStatus {
    pub fn id(&self) -> i64 {
        *self as i64
    }
}

#[repr(i64)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
#[serde(rename_all = "camelCase")]
pub enum AssetType {
    Poster = 1,
    Backdrop = 2,
    Logo = 3,
    Banner = 4,
    Thumbnail = 5,
    Icon = 6,
    Trailer = 7,
    Screenshot = 8,
}

impl AssetType {
    pub fn id(self) -> i64 {
        self as i64
    }
}

impl TryFrom<i64> for AssetType {
    type Error = MediaError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(AssetType::Poster),
            2 => Ok(AssetType::Backdrop),
            3 => Ok(AssetType::Logo),
            4 => Ok(AssetType::Banner),
            5 => Ok(AssetType::Thumbnail),
            6 => Ok(AssetType::Icon),
            7 => Ok(AssetType::Trailer),
            8 => Ok(AssetType::Screenshot),
            _ => Err(MediaError::InvalidAssetType),
        }
    }
}
