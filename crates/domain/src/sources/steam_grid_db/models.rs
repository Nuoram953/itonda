use serde::{Deserialize, Serialize};

use crate::{assets::types::AssetType, media::discovered::DiscoveredAsset};

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize)]
pub struct GridSearchOptions {
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub dimensions: Option<String>,
    #[serde(rename = "type")]
    pub types: Option<String>,
    pub styles: Option<String>,
    pub mimes: Option<String>,
}

impl GridSearchOptions {
    pub fn poster(page: u32, limit: u32) -> Self {
        Self {
            page: Some(page),
            limit: Some(limit),
            styles: Some("alternate,material".into()),
            dimensions: Some("600x900".into()),
            mimes: Some("image/jpeg,image/png".into()),
            types: Some("static".into()),
        }
    }

    pub fn hero(page: u32, limit: u32) -> Self {
        Self {
            page: Some(page),
            limit: Some(limit),
            styles: Some("alternate,material".into()),
            dimensions: Some("1920x620,3840x1240".into()),
            mimes: Some("image/jpeg,image/png,image/webp".into()),
            types: Some("static".into()),
        }
    }

    pub fn icon(page: u32, limit: u32) -> Self {
        Self {
            page: Some(page),
            limit: Some(limit),
            styles: Some("official,custom".into()),
            mimes: Some("image/png".into()),
            types: Some("static".into()),
            ..Default::default()
        }
    }

    pub fn logo(page: u32, limit: u32) -> Self {
        Self {
            page: Some(page),
            limit: Some(limit),
            styles: Some("official,white,black".into()),
            mimes: Some("image/png,image/webp".into()),
            types: Some("static".into()),
            ..Default::default()
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct MediaResponse {
    pub success: bool,
    pub page: u32,
    pub total: u32,
    pub limit: u32,
    pub data: Vec<Media>,
}

impl MediaResponse {
    pub fn into_assets(self, asset_type: AssetType) -> Vec<DiscoveredAsset> {
        self.into_assets_with_game_id(asset_type, None)
    }

    pub fn into_assets_with_game_id(
        self,
        asset_type: AssetType,
        game_id: Option<u32>,
    ) -> Vec<DiscoveredAsset> {
        let provider_external_id = game_id.map(|id| crate::media::models::MediaExternalId {
            provider: crate::media::models::ExternalIdProvider::SteamGridDb,
            external_id: id.to_string(),
        });
        self.data
            .into_iter()
            .map(|media| DiscoveredAsset {
                asset_type,
                url: media.url,
                provider_external_id: provider_external_id.clone(),
                pillar_id: None,
            })
            .collect()
    }
}

#[derive(Debug, Deserialize)]
pub struct Media {
    pub id: u32,
    pub score: u32,
    pub style: String,
    pub mime: String,
    pub url: String,
    pub thumb: String,
    pub author: MediaAuthor,
}

#[derive(Debug, Deserialize)]
pub struct MediaAuthor {
    pub name: String,
    pub steam64: String,
    pub avatar: String,
}

#[derive(Debug, Deserialize)]
pub struct GetExternalGameIdResponse {
    pub success: bool,
    pub data: SteamGridDbGame,
}

#[derive(Debug, Deserialize)]
pub struct SteamGridDbGame {
    pub id: u32,
    pub name: String,
    pub types: Vec<String>,
    pub verified: bool,
}

#[derive(Debug, Deserialize)]
pub struct SearchResponse {
    pub success: bool,
    pub data: Vec<SteamGridDbGame>,
}
