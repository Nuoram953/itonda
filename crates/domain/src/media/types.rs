use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::media::errors::MediaError;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MediaType {
    Game,
    Movie,
    TvShow,
}

impl MediaType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Game => "game",
            Self::Movie => "movie",
            Self::TvShow => "tv_show",
        }
    }
}

impl TryFrom<String> for MediaType {
    type Error = MediaError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "game" => Ok(Self::Game),
            "movie" => Ok(Self::Movie),
            "tv_show" => Ok(Self::TvShow),
            _ => Err(MediaError::InvalidMediaType),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema)]
#[repr(i64)]
#[serde(rename_all = "snake_case")]
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

impl TryFrom<i64> for MediaStatus {
    type Error = MediaError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::NotStarted),
            2 => Ok(Self::InProgress),
            3 => Ok(Self::Completed),
            4 => Ok(Self::Abandoned),
            5 => Ok(Self::Paused),
            _ => Err(MediaError::InvalidMediaStatus),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
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

impl TryFrom<&str> for MediaLaunchType {
    type Error = MediaError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "storefront" | "steam" => Ok(Self::Storefront),
            "emulator" => Ok(Self::Emulator),
            "custom" => Ok(Self::Custom),
            _ => Err(MediaError::InvalidLaunchType),
        }
    }
}

impl TryFrom<String> for MediaLaunchType {
    type Error = MediaError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum MediaSource {
    Steam,
}

impl MediaSource {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Steam => "steam",
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MediaSortField {
    Title,
    LastPlayedAt,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SortOrder {
    Asc,
    Desc,
}
