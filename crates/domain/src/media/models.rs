use itonda_database::media::MediaRow;
use serde::{Deserialize, Serialize};

use utoipa::ToSchema;

use crate::{media::errors::MediaError, storefronts::models::StorefrontId};

pub struct DiscoveredMedia {
    pub storefront: StorefrontId,
    pub external_id: String,
    pub media_type: MediaType,
    pub title: String,
    pub metadata: DiscoveredMediaMetadata,
}

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

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum MediaSource {
    Steam,
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
