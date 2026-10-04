use async_trait::async_trait;

use crate::{
    assets::{
        error::AssetError,
        models::{AssetStoreId, PosterSearchOptions},
        traits::{AssetFetcher, PosterFetcher},
    },
    media::{discovered::DiscoveredAsset, types::MediaType},
    metadata::{error::MetadataError, models::MediaSearchResult, traits::MediaSearcher},
    sources::the_movie_database::client::TheMovieDatabaseClient,
    storefronts::models::StorefrontId,
};

pub mod client;
pub mod models;

#[cfg(test)]
mod tests;

pub struct TheMovieDatabase {
    client: TheMovieDatabaseClient,
}

impl TheMovieDatabase {
    pub fn new(api_key: String) -> Self {
        Self {
            client: TheMovieDatabaseClient::new(api_key),
        }
    }

    pub fn supports_media_type(&self, media_type: MediaType) -> bool {
        matches!(media_type, MediaType::TvShow | MediaType::Movie)
    }
}

impl AssetFetcher for TheMovieDatabase {
    fn id(&self) -> AssetStoreId {
        AssetStoreId::TheMovieDatabase
    }

    fn supports_media_type(&self, media_type: MediaType) -> bool {
        matches!(media_type, MediaType::TvShow | MediaType::Movie)
    }
}

#[async_trait]
impl PosterFetcher for TheMovieDatabase {
    async fn discover_poster(
        &self,
        media_type: Option<MediaType>,
        _storefront: Option<StorefrontId>,
        _external_id: Option<&str>,
        title: &str,
    ) -> Result<Option<DiscoveredAsset>, AssetError> {
        let opts = PosterSearchOptions::Default;
        Ok(self
            .search_poster(media_type, None, None, title, &opts)
            .await?
            .into_iter()
            .next())
    }

    async fn search_poster(
        &self,
        media_type: Option<MediaType>,
        _storefront: Option<StorefrontId>,
        _external_id: Option<&str>,
        title: &str,
        _options: &PosterSearchOptions,
    ) -> Result<Vec<DiscoveredAsset>, AssetError> {
        let Some((tmdb_type, media_id)) = self
            .client
            .find_media_id(media_type.as_ref(), title)
            .await?
        else {
            return Ok(Vec::new());
        };

        let response = self.client.get_media_images(tmdb_type, media_id).await?;

        Ok(response.into_poster_assets())
    }
}

#[async_trait]
impl MediaSearcher for TheMovieDatabase {
    fn supports_media_type(&self, media_type: MediaType) -> bool {
        matches!(media_type, MediaType::Movie | MediaType::TvShow)
    }

    async fn search(
        &self,
        query: &str,
        media_type: MediaType,
    ) -> Result<Vec<MediaSearchResult>, MetadataError> {
        match media_type {
            MediaType::Movie => {
                let movies = self
                    .client
                    .search_movie_results(query)
                    .await
                    .map_err(|e| MetadataError::Other(e.to_string()))?;
                Ok(movies.into_iter().map(|m| m.into_search_result()).collect())
            }
            MediaType::TvShow => {
                let tv_shows = self
                    .client
                    .search_tv_results(query)
                    .await
                    .map_err(|e| MetadataError::Other(e.to_string()))?;
                Ok(tv_shows
                    .into_iter()
                    .map(|t| t.into_search_result())
                    .collect())
            }
            _ => Ok(Vec::new()),
        }
    }
}
