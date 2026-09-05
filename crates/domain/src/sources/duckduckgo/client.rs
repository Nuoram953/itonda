use regex::Regex;
use reqwest_middleware::ClientWithMiddleware;
use std::sync::LazyLock;
use std::time::Duration;
use tokio::time::sleep;
use tracing::warn;

use crate::{
    http::{RateLimiter, create_rate_limited_http_client},
    sources::duckduckgo::models::{
        DuckDuckGoError, DuckDuckGoImageResult, DuckDuckGoSearchResponse,
    },
};

static RE_VQD: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"vqd=([\d-]+)"#).unwrap());

const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36";
const SEC_CH_UA: &str = r#""Not(A:Brand";v="99", "Google Chrome";v="133", "Chromium";v="133""#;

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
        Self::with_rate_limiter(RateLimiter::new(1, 1.0))
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
        let max_retries = 2;
        let mut attempt = 0;

        loop {
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
                .header("User-Agent", USER_AGENT)
                .header("Accept", "application/json, text/javascript, */*; q=0.01")
                .header("Accept-Language", "en-US,en;q=0.9")
                .header("Referer", "https://duckduckgo.com/")
                .header("sec-ch-ua", SEC_CH_UA)
                .header("sec-ch-ua-mobile", "?0")
                .header("sec-ch-ua-platform", r#""Windows""#)
                .header("sec-fetch-dest", "empty")
                .header("sec-fetch-mode", "cors")
                .header("sec-fetch-site", "same-origin")
                .header("x-requested-with", "XMLHttpRequest")
                .send()
                .await?;

            let status = response.status();
            if status.is_success() {
                let payload = response.json::<DuckDuckGoSearchResponse>().await?;
                return Ok(payload.results);
            }

            if (status == http::StatusCode::FORBIDDEN || status == http::StatusCode::TOO_MANY_REQUESTS)
                && attempt < max_retries
            {
                attempt += 1;
                let backoff_secs = 1.5 * (attempt as f64);
                warn!(
                    status = %status,
                    attempt,
                    backoff_secs,
                    query,
                    "DuckDuckGo returned throttling error, backing off before retry"
                );
                sleep(Duration::from_secs_f64(backoff_secs)).await;
                continue;
            }

            if status == http::StatusCode::FORBIDDEN {
                warn!(query, "DuckDuckGo image search forbidden (403)");
                return Err(DuckDuckGoError::Forbidden);
            }

            if status == http::StatusCode::TOO_MANY_REQUESTS {
                warn!(query, "DuckDuckGo image search rate limited (429)");
                return Err(DuckDuckGoError::RateLimited);
            }

            warn!(
                status = %status,
                query,
                "DuckDuckGo search returned non-success status"
            );
            return Ok(Vec::new());
        }
    }

    async fn extract_vqd(&self, query: &str) -> Result<String, DuckDuckGoError> {
        let response = self
            .client
            .get(&self.vqd_base_url)
            .query(&[("q", query), ("iar", "images"), ("iax", "images"), ("ia", "images")])
            .header("User-Agent", USER_AGENT)
            .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8")
            .header("Accept-Language", "en-US,en;q=0.9")
            .header("sec-ch-ua", SEC_CH_UA)
            .header("sec-ch-ua-mobile", "?0")
            .header("sec-ch-ua-platform", r#""Windows""#)
            .header("sec-fetch-dest", "document")
            .header("sec-fetch-mode", "navigate")
            .header("sec-fetch-site", "none")
            .header("sec-fetch-user", "?1")
            .header("upgrade-insecure-requests", "1")
            .send()
            .await?;

        let status = response.status();
        if status == http::StatusCode::FORBIDDEN {
            warn!(query, "DuckDuckGo VQD request forbidden (403)");
            return Err(DuckDuckGoError::Forbidden);
        }
        if status == http::StatusCode::TOO_MANY_REQUESTS {
            warn!(query, "DuckDuckGo VQD request rate limited (429)");
            return Err(DuckDuckGoError::RateLimited);
        }

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

