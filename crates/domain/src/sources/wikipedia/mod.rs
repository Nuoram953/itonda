pub mod cleaner;
pub mod client;
pub mod models;
pub mod parser;

#[cfg(test)]
mod tests;

pub use client::WikipediaClient;
pub use models::ParsedPillar;

use async_trait::async_trait;

use crate::{
    media::{models::GameplayPillar, types::MediaType},
    metadata::{
        error::MetadataError,
        models::{
            CommonMetadata, GameGeneralMetadata, GeneralMetadata, MetadataProviderId,
            MetadataQuery,
        },
        traits::{GeneralInfoFetcher, MetadataFetcher},
    },
};

pub struct WikipediaSource {
    client: WikipediaClient,
}

impl Default for WikipediaSource {
    fn default() -> Self {
        Self::new()
    }
}

impl WikipediaSource {
    pub fn new() -> Self {
        Self {
            client: WikipediaClient::new(),
        }
    }

    pub fn with_rate_limiter(rate_limiter: crate::http::RateLimiter) -> Self {
        Self {
            client: WikipediaClient::with_rate_limiter(rate_limiter),
        }
    }

    pub fn client(&self) -> &WikipediaClient {
        &self.client
    }


    pub async fn fetch_gameplay_pillars(
        &self,
        title: &str,
    ) -> Result<Vec<ParsedPillar>, client::WikipediaError> {
        self.client.fetch_gameplay_pillars(title).await
    }
}

impl MetadataFetcher for WikipediaSource {
    fn id(&self) -> MetadataProviderId {
        MetadataProviderId::Wikipedia
    }

    fn name(&self) -> &'static str {
        "Wikipedia"
    }

    fn supports_media_type(&self, media_type: MediaType) -> bool {
        matches!(media_type, MediaType::Game)
    }
}

#[async_trait]
impl GeneralInfoFetcher for WikipediaSource {
    async fn fetch_general_info(
        &self,
        query: &MetadataQuery<'_>,
    ) -> Result<Option<GeneralMetadata>, MetadataError> {
        let pillars = self
            .fetch_gameplay_pillars(query.title)
            .await
            .map_err(|e| MetadataError::Other(e.to_string()))?;


        if pillars.is_empty() {
            return Ok(None);
        }

        let domain_pillars = pillars
            .into_iter()
            .map(|p| GameplayPillar {
                id: p.id,
                title: p.title,
                description: p.description,
                icon: p.icon,
                asset_id: None,
            })
            .collect();

        Ok(Some(GeneralMetadata::Game(GameGeneralMetadata {
            common: CommonMetadata::default(),
            developers: Vec::new(),
            publishers: Vec::new(),
            platforms: Vec::new(),
            series: None,
            pillars: domain_pillars,
        })))
    }
}

