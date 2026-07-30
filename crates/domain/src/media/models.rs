use itonda_database::media::MediaRow;
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
