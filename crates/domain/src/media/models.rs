use itonda_database::media::{
    MediaAssetRow, MediaExternalIdRow, MediaGameDetailsRow, MediaInstallationRow, MediaLaunchRow,
    MediaRow, MediaStorefrontRow,
};
use serde::{Deserialize, Serialize};

use utoipa::ToSchema;

use crate::{
    assets::{error::AssetError, types::AssetType},
    media::{
        errors::MediaError,
        types::{MediaStatus, MediaType},
    },
    storefronts::models::StorefrontId,
};

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Media {
    pub id: String,
    pub title: String,
    pub media_type: MediaType,
    pub status: MediaStatus,
    pub description: Option<String>,
    pub summary: Option<String>,
    pub release_date: Option<i64>,
    pub genres: Vec<String>,
    pub tags: Vec<String>,
    pub assets: Vec<Asset>,
    pub details: Option<MediaDetails>,
    pub storefronts: Vec<MediaStorefront>,
    pub external_ids: Vec<MediaExternalId>,
    pub installations: Vec<MediaInstallation>,
    pub launches: Vec<Launch>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExternalIdProvider {
    Steam,
    Igdb,
    SteamGridDb,
    Tmdb,
}

impl ExternalIdProvider {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Steam => "steam",
            Self::Igdb => "igdb",
            Self::SteamGridDb => "steamgriddb",
            Self::Tmdb => "tmdb",
        }
    }
}

impl TryFrom<&str> for ExternalIdProvider {
    type Error = MediaError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "steam" => Ok(Self::Steam),
            "igdb" => Ok(Self::Igdb),
            "steamgriddb" => Ok(Self::SteamGridDb),
            "tmdb" => Ok(Self::Tmdb),
            _ => Err(MediaError::InvalidExternalProvider(value.to_string())),
        }
    }
}

impl TryFrom<String> for ExternalIdProvider {
    type Error = MediaError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
pub struct MediaExternalId {
    pub provider: ExternalIdProvider,
    pub external_id: String,
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
    pub agent_id: Option<String>,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
pub struct MediaStorefront {
    pub storefront_id: StorefrontId,
    pub external_id: String,
    pub playtime_minutes: Option<i64>,
    pub last_played_at: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
pub struct MediaInstallation {
    pub id: String,
    pub agent_id: String,
    pub storefront_id: Option<StorefrontId>,
    pub external_id: Option<String>,
    pub path: Option<String>,
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
    pub series: Option<String>,
    pub developers: Vec<String>,
    pub publishers: Vec<String>,
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
            agent_id: row.agent_id,
            name: row.name,
        }
    }
}

impl TryFrom<MediaStorefrontRow> for MediaStorefront {
    type Error = MediaError;

    fn try_from(row: MediaStorefrontRow) -> Result<Self, Self::Error> {
        Ok(Self {
            storefront_id: StorefrontId::try_from(row.storefront_id)?,
            external_id: row.external_id,
            playtime_minutes: row.playtime_minutes,
            last_played_at: row.last_played_at,
        })
    }
}

impl TryFrom<MediaInstallationRow> for MediaInstallation {
    type Error = MediaError;

    fn try_from(row: MediaInstallationRow) -> Result<Self, Self::Error> {
        let storefront_id = match row.storefront_id {
            Some(sf) => Some(StorefrontId::try_from(sf)?),
            None => None,
        };

        Ok(Self {
            id: row.id,
            agent_id: row.agent_id,
            storefront_id,
            external_id: row.external_id,
            path: row.path,
        })
    }
}

impl TryFrom<MediaExternalIdRow> for MediaExternalId {
    type Error = MediaError;

    fn try_from(row: MediaExternalIdRow) -> Result<Self, Self::Error> {
        Ok(Self {
            provider: ExternalIdProvider::try_from(row.provider.as_str())?,
            external_id: row.external_id,
        })
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
            description: row.description,
            summary: row.summary,
            release_date: row.release_date,
            genres: Vec::new(),
            tags: Vec::new(),
            assets: Vec::new(),
            storefronts: Vec::new(),
            external_ids: Vec::new(),
            installations: Vec::new(),
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
            series: row.series,
            developers: Vec::new(),
            publishers: Vec::new(),
        }
    }
}
