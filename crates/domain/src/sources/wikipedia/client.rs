use reqwest_middleware::ClientWithMiddleware;
use thiserror::Error;

use crate::{
    http::{RateLimiter, create_rate_limited_http_client},
    sources::wikipedia::{
        models::{
            ParsedPillar, WikipediaImageInfoResponse, WikipediaSectionsResponse,
            WikipediaWikitextResponse,
        },
        parser::parse_gameplay_wikitext,
    },
};

#[derive(Debug, Error)]
pub enum WikipediaError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest_middleware::Error),
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("JSON parse error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Page not found: {0}")]
    NotFound(String),
}

pub struct WikipediaClient {
    client: ClientWithMiddleware,
    base_url: String,
}

impl Default for WikipediaClient {
    fn default() -> Self {
        Self::new()
    }
}

impl WikipediaClient {
    pub fn new() -> Self {
        Self::with_rate_limiter(RateLimiter::new(6, 2.0))
    }

    pub fn with_rate_limiter(rate_limiter: RateLimiter) -> Self {
        Self {
            client: create_rate_limited_http_client(rate_limiter),
            base_url: "https://en.wikipedia.org/w/api.php".into(),
        }
    }

    #[cfg(test)]
    pub fn with_base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into();
        self
    }


    pub async fn fetch_gameplay_pillars(
        &self,
        title: &str,
    ) -> Result<Vec<ParsedPillar>, WikipediaError> {
        let candidates = vec![
            title.to_string(),
            format!("{} (video game)", title),
            format!("{} (game)", title),
        ];

        for candidate in candidates {
            if let Some(section_idx) = self.find_gameplay_section(&candidate).await? {
                if let Some(wikitext) = self.get_section_wikitext(&candidate, &section_idx).await? {
                    let pillars = parse_gameplay_wikitext(&wikitext);
                    if !pillars.is_empty() {
                        return Ok(pillars);
                    }
                }
            }
        }

        Ok(Vec::new())
    }

    pub async fn find_gameplay_section(
        &self,
        page_title: &str,
    ) -> Result<Option<String>, WikipediaError> {
        let response = self
            .client
            .get(&self.base_url)
            .query(&[
                ("action", "parse"),
                ("page", page_title),
                ("prop", "sections"),
                ("redirects", "1"),
                ("format", "json"),
            ])
            .header("User-Agent", "ItondaMediaManager/1.0 (https://github.com/itonda)")
            .send()
            .await?;

        if !response.status().is_success() {
            return Ok(None);
        }

        let payload = response.json::<WikipediaSectionsResponse>().await?;
        let Some(parse) = payload.parse else {
            return Ok(None);
        };

        for section in parse.sections {
            let line_lower = section.line.to_lowercase();
            if line_lower == "gameplay" || line_lower == "game play" || line_lower.starts_with("gameplay") {
                return Ok(Some(section.index));
            }
        }

        Ok(None)
    }

    pub async fn get_section_wikitext(
        &self,
        page_title: &str,
        section_index: &str,
    ) -> Result<Option<String>, WikipediaError> {
        let response = self
            .client
            .get(&self.base_url)
            .query(&[
                ("action", "parse"),
                ("page", page_title),
                ("prop", "wikitext"),
                ("section", section_index),
                ("redirects", "1"),
                ("format", "json"),
            ])
            .header("User-Agent", "ItondaMediaManager/1.0 (https://github.com/itonda)")
            .send()
            .await?;

        if !response.status().is_success() {
            return Ok(None);
        }

        let payload = response.json::<WikipediaWikitextResponse>().await?;
        let Some(parse) = payload.parse else {
            return Ok(None);
        };

        Ok(parse.wikitext.map(|w| w.content))
    }

    pub async fn fetch_image_url(&self, file_name: &str) -> Result<Option<String>, WikipediaError> {
        let title_param = if file_name.starts_with("File:") || file_name.starts_with("Image:") {
            file_name.to_string()
        } else {
            format!("File:{}", file_name)
        };

        let response = self
            .client
            .get(&self.base_url)
            .query(&[
                ("action", "query"),
                ("titles", &title_param),
                ("prop", "imageinfo"),
                ("iiprop", "url"),
                ("format", "json"),
            ])
            .header("User-Agent", "ItondaMediaManager/1.0 (https://github.com/itonda)")
            .send()
            .await?;

        if !response.status().is_success() {
            return Ok(None);
        }

        let payload = response.json::<WikipediaImageInfoResponse>().await?;
        if let Some(query) = payload.query {
            if let Some(pages) = query.pages {
                for (_page_id, page_item) in pages {
                    if let Some(imageinfo) = page_item.imageinfo {
                        if let Some(first) = imageinfo.first() {
                            if let Some(url) = &first.url {
                                return Ok(Some(url.clone()));
                            }
                        }
                    }
                }
            }
        }

        Ok(None)
    }
}
