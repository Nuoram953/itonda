use async_trait::async_trait;

use crate::{
    assets::{
        error::AssetError,
        models::{AssetStoreId, PosterSearchOptions},
        traits::{AssetFetcher, PosterFetcher},
        types::AssetType,
    },
    media::{
        discovered::DiscoveredAsset,
        models::{ExternalIdProvider, MediaExternalId},
        types::MediaType,
    },
    metadata::{
        error::MetadataError,
        models::{CommonMetadata, GeneralMetadata, MediaSearchResult, MetadataProviderId, MetadataQuery},
        traits::{GeneralInfoFetcher, MediaSearcher, MetadataFetcher},
    },
    sources::youtube::client::YoutubeClient,
    storefronts::models::StorefrontId,
};

pub mod client;
pub mod models;

#[cfg(test)]
mod tests;

#[derive(Clone, Debug, Default)]
pub struct YoutubeSource {
    client: YoutubeClient,
}

impl YoutubeSource {
    pub fn new() -> Self {
        Self {
            client: YoutubeClient::new(),
        }
    }

    pub fn with_client(client: YoutubeClient) -> Self {
        Self { client }
    }

    pub fn client(&self) -> &YoutubeClient {
        &self.client
    }

    pub fn supports_media_type(&self, _media_type: MediaType) -> bool {
        true
    }

    fn parse_year(upload_date: Option<&str>) -> Option<u32> {
        upload_date
            .and_then(|d| d.get(0..4))
            .and_then(|y| y.parse::<u32>().ok())
    }

    fn parse_release_timestamp(upload_date: Option<&str>) -> Option<i64> {
        let date_str = upload_date?;
        if date_str.len() == 8 {
            chrono::NaiveDate::parse_from_str(date_str, "%Y%m%d")
                .ok()
                .and_then(|d| d.and_hms_opt(0, 0, 0))
                .map(|dt| dt.and_utc().timestamp())
        } else {
            None
        }
    }
}

impl AssetFetcher for YoutubeSource {
    fn id(&self) -> AssetStoreId {
        AssetStoreId::Youtube
    }

    fn supports_media_type(&self, _media_type: MediaType) -> bool {
        true
    }
}

#[async_trait]
impl PosterFetcher for YoutubeSource {
    async fn discover_poster(
        &self,
        media_type: Option<MediaType>,
        _storefront: Option<StorefrontId>,
        external_id: Option<&str>,
        title: &str,
    ) -> Result<Option<DiscoveredAsset>, AssetError> {
        let options = PosterSearchOptions::Default;
        let posters = self
            .search_poster(media_type, None, external_id, title, &options)
            .await?;
        Ok(posters.into_iter().next())
    }

    async fn search_poster(
        &self,
        media_type: Option<MediaType>,
        _storefront: Option<StorefrontId>,
        external_id: Option<&str>,
        title: &str,
        _options: &PosterSearchOptions,
    ) -> Result<Vec<DiscoveredAsset>, AssetError> {
        if let Some(mt) = media_type
            && !self.supports_media_type(mt)
        {
            return Ok(Vec::new());
        }

        // If we have an external ID, fetch its thumbnail
        if let Some(id) = external_id {
            match self.client.get_video_info(id).await {
                Ok(info) => {
                    if let Some(thumb) = info.thumbnail {
                        return Ok(vec![
                            DiscoveredAsset::new(AssetType::Poster, thumb)
                                .with_provider_external_id(MediaExternalId {
                                    provider: ExternalIdProvider::Youtube,
                                    external_id: info.id,
                                }),
                        ]);
                    }
                }
                Err(err) => {
                    tracing::warn!("Failed to get video info for poster discovery: {err}");
                }
            }
        }

        // If title is a search query or URL
        if let Ok(results) = self.client.search(title, 1).await {
            if let Some(first) = results.into_iter().next() {
                if let Some(thumb) = first.thumbnail {
                    return Ok(vec![
                        DiscoveredAsset::new(AssetType::Poster, thumb)
                            .with_provider_external_id(MediaExternalId {
                                provider: ExternalIdProvider::Youtube,
                                external_id: first.id,
                            }),
                    ]);
                }
            }
        }

        Ok(Vec::new())
    }
}

#[async_trait]
impl MetadataFetcher for YoutubeSource {
    fn id(&self) -> MetadataProviderId {
        MetadataProviderId::Youtube
    }

    fn name(&self) -> &'static str {
        "YouTube"
    }

    fn supports_media_type(&self, _media_type: MediaType) -> bool {
        true
    }
}

#[async_trait]
impl GeneralInfoFetcher for YoutubeSource {
    async fn fetch_general_info(
        &self,
        query: &MetadataQuery<'_>,
    ) -> Result<Option<GeneralMetadata>, MetadataError> {
        if !self.supports_media_type(query.media_type) {
            return Ok(None);
        }

        let extracted_id = YoutubeClient::extract_video_id(query.title);
        let youtube_id = query
            .external_ids
            .iter()
            .find(|e| e.provider == ExternalIdProvider::Youtube)
            .map(|e| e.external_id.as_str())
            .or(query.external_id)
            .or(extracted_id.as_deref());

        let video_info = if let Some(id) = youtube_id {
            self.client.get_video_info(id).await.ok()
        } else {
            self.client
                .search(query.title, 1)
                .await
                .ok()
                .and_then(|r| r.into_iter().next())
        };

        let Some(info) = video_info else {
            return Ok(None);
        };

        let release_date = Self::parse_release_timestamp(info.upload_date.as_deref());
        let common = CommonMetadata {
            description: info.description,
            summary: info.uploader,
            release_date,
            genres: Vec::new(),
            tags: info.tags,
            external_ids: vec![MediaExternalId {
                provider: ExternalIdProvider::Youtube,
                external_id: info.id,
            }],
        };

        Ok(Some(GeneralMetadata::Common(common)))
    }
}

#[async_trait]
impl MediaSearcher for YoutubeSource {
    fn supports_media_type(&self, _media_type: MediaType) -> bool {
        true
    }

    async fn search(
        &self,
        query: &str,
        media_type: MediaType,
    ) -> Result<Vec<MediaSearchResult>, MetadataError> {
        if !self.supports_media_type(media_type) {
            return Ok(Vec::new());
        }

        let results = self
            .client
            .search(query, 10)
            .await
            .map_err(|err| MetadataError::Other(err.to_string()))?;

        Ok(results
            .into_iter()
            .map(|info| {
                let year = Self::parse_year(info.upload_date.as_deref());
                MediaSearchResult {
                    external_id: info.id,
                    title: info.title,
                    media_type,
                    year,
                    summary: info.uploader.or(info.description),
                    cover_url: info.thumbnail,
                }
            })
            .collect())
    }
}
