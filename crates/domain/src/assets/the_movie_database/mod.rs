use async_trait::async_trait;

use crate::{
    assets::{
        error::AssetError,
        models::{AssetStoreId, PosterSearchOptions},
        the_movie_database::client::TheMovieDatabaseClient,
        traits::{AssetFetcher, PosterFetcher},
    },
    media::{discovered::DiscoveredAsset, types::MediaType},
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
