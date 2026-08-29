use regex::Regex;
use reqwest_middleware::ClientWithMiddleware;
use std::sync::LazyLock;

use crate::{
    http::{RateLimiter, create_rate_limited_http_client},
    sources::duckduckgo::models::{
        DuckDuckGoError, DuckDuckGoImageResult, DuckDuckGoSearchResponse,
    },
};

static RE_VQD: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"vqd=([\d-]+)"#).unwrap());

pub struct DuckDuckGoImageClient {
    client: ClientWithMiddleware,
    search_base_url: String,
    vqd_base_url: String,
}

impl Default for DuckDuckGoImageClient {
    fn default() -> Self {
        Self::new()
    }
}

impl DuckDuckGoImageClient {
    pub fn new() -> Self {
        Self::with_rate_limiter(RateLimiter::new(4, 2.0))
    }

    pub fn with_rate_limiter(rate_limiter: RateLimiter) -> Self {
        Self {
            client: create_rate_limited_http_client(rate_limiter),
            search_base_url: "https://duckduckgo.com/i.js".into(),
            vqd_base_url: "https://duckduckgo.com/".into(),
        }
    }

    #[cfg(test)]
    pub fn with_urls(mut self, vqd_url: impl Into<String>, search_url: impl Into<String>) -> Self {
        self.vqd_base_url = vqd_url.into();
        self.search_base_url = search_url.into();
        self
    }

    pub async fn search_first_image(&self, query: &str) -> Result<Option<String>, DuckDuckGoError> {
        let results = self.search_images(query).await?;
        Ok(results.into_iter().next().map(|r| r.image))
    }

    pub async fn search_images(&self, query: &str) -> Result<Vec<DuckDuckGoImageResult>, DuckDuckGoError> {
        self.search_images_with_filter(query, ",,,").await
    }

    pub async fn search_images_with_filter(
        &self,
        query: &str,
        filter: &str,
    ) -> Result<Vec<DuckDuckGoImageResult>, DuckDuckGoError> {
        let vqd = self.extract_vqd(query).await?;

        let response = self
            .client
            .get(&self.search_base_url)
            .query(&[
                ("l", "us-en"),
                ("o", "json"),
                ("q", query),
                ("vqd", &vqd),
                ("f", filter),
                ("p", "1"),
            ])
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
            .header("Accept", "application/json, text/javascript, */*; q=0.01")
            .header("Accept-Language", "en-US,en;q=0.5")
            .header("Referer", "https://duckduckgo.com/")
            .header("Sec-Fetch-Dest", "empty")
            .header("Sec-Fetch-Mode", "cors")
            .header("Sec-Fetch-Site", "same-origin")
            .header("X-Requested-With", "XMLHttpRequest")
            .send()
            .await?;

        if !response.status().is_success() {
            return Ok(Vec::new());
        }

        let payload = response.json::<DuckDuckGoSearchResponse>().await?;
        Ok(payload.results)
    }

    async fn extract_vqd(&self, query: &str) -> Result<String, DuckDuckGoError> {
        let response = self
            .client
            .get(&self.vqd_base_url)
            .query(&[("q", query), ("iar", "images"), ("iax", "images"), ("ia", "images")])
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
            .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8")
            .header("Accept-Language", "en-US,en;q=0.5")
            .send()
            .await?;

        let text = response.text().await?;

        if let Some(caps) = RE_VQD.captures(&text) {
            if let Some(matched) = caps.get(1) {
                return Ok(matched.as_str().to_string());
            }
        }

        let fallback_re = Regex::new(r#"vqd=["']([^"']+)["']"#).unwrap();
        if let Some(caps) = fallback_re.captures(&text) {
            if let Some(matched) = caps.get(1) {
                return Ok(matched.as_str().to_string());
            }
        }

        Err(DuckDuckGoError::VqdNotFound)
    }
}
