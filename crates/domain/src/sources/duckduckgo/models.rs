use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DuckDuckGoError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest_middleware::Error),
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("VQD token not found in response")]
    VqdNotFound,
    #[error("DuckDuckGo request forbidden or blocked (status 403)")]
    Forbidden,
    #[error("DuckDuckGo rate limited (status 429)")]
    RateLimited,
    #[error("No image results found")]
    NoResults,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct DuckDuckGoImageResult {
    pub image: String,
    pub title: String,
    #[serde(default)]
    pub width: u32,
    #[serde(default)]
    pub height: u32,
}

#[derive(Debug, Deserialize)]
pub struct DuckDuckGoSearchResponse {
    #[serde(default)]
    pub results: Vec<DuckDuckGoImageResult>,
}
