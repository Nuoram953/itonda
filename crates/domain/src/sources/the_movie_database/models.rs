use serde::{Deserialize, Serialize};

use crate::{assets::types::AssetType, media::discovered::DiscoveredAsset};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TmdbMediaType {
    Movie,
    Tv,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TmdbMovieSearchResponse {
    pub page: Option<u32>,
    #[serde(default)]
    pub results: Vec<TmdbMovieResult>,
    pub total_pages: Option<u32>,
    pub total_results: Option<u32>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TmdbMovieResult {
    pub id: u64,
    pub title: Option<String>,
    pub original_title: Option<String>,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    pub overview: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TmdbTvSearchResponse {
    pub page: Option<u32>,
    #[serde(default)]
    pub results: Vec<TmdbTvResult>,
    pub total_pages: Option<u32>,
    pub total_results: Option<u32>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TmdbTvResult {
    pub id: u64,
    pub name: Option<String>,
    pub original_name: Option<String>,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    pub overview: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TmdbMultiSearchResponse {
    pub page: Option<u32>,
    #[serde(default)]
    pub results: Vec<TmdbMultiResult>,
    pub total_pages: Option<u32>,
    pub total_results: Option<u32>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TmdbMultiResult {
    pub id: u64,
    pub media_type: String,
    pub title: Option<String>,
    pub name: Option<String>,
    pub poster_path: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TmdbKeywordSearchResponse {
    pub page: Option<u32>,
    #[serde(default)]
    pub results: Vec<TmdbKeywordResult>,
    pub total_pages: Option<u32>,
    pub total_results: Option<u32>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TmdbKeywordResult {
    pub id: u64,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TmdbImagesResponse {
    pub id: u64,
    #[serde(default)]
    pub backdrops: Vec<TmdbImageItem>,
    #[serde(default)]
    pub posters: Vec<TmdbImageItem>,
    #[serde(default)]
    pub logos: Vec<TmdbImageItem>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TmdbImageItem {
    pub aspect_ratio: Option<f64>,
    pub height: Option<u32>,
    pub width: Option<u32>,
    pub file_path: String,
    pub iso_639_1: Option<String>,
    pub vote_average: Option<f64>,
    pub vote_count: Option<u32>,
}

impl TmdbImagesResponse {
    pub fn into_poster_assets(self) -> Vec<DiscoveredAsset> {
        let provider_external_id = Some(crate::media::models::MediaExternalId {
            provider: crate::media::models::ExternalIdProvider::Tmdb,
            external_id: self.id.to_string(),
        });
        self.posters
            .into_iter()
            .map(|item| DiscoveredAsset {
                asset_type: AssetType::Poster,
                url: format!("https://image.tmdb.org/t/p/original{}", item.file_path),
                provider_external_id: provider_external_id.clone(),
            })
            .collect()
    }
}
