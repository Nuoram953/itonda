use async_trait::async_trait;

use crate::{
    assets::{
        error::AssetError,
        models::{AssetStoreId, PosterSearchOptions},
        traits::{AssetFetcher, BannerFetcher, PosterFetcher},
        types::AssetType,
    },
    media::{discovered::DiscoveredAsset, types::MediaType},
    sources::steam_grid_db::{client::SteamGridDbClient, models::GridSearchOptions},
    storefronts::models::StorefrontId,
};

pub mod client;
pub mod models;

#[cfg(test)]
mod tests;

pub struct SteamGridDb {
    client: SteamGridDbClient,
}

impl SteamGridDb {
    pub fn new(api_key: String) -> Self {
        Self {
            client: SteamGridDbClient::new(api_key),
        }
    }
}

impl AssetFetcher for SteamGridDb {
    fn id(&self) -> AssetStoreId {
        AssetStoreId::SteamGridDb
    }

    fn supports_media_type(&self, media_type: MediaType) -> bool {
        matches!(media_type, MediaType::Game)
    }
}

#[async_trait]
impl PosterFetcher for SteamGridDb {
    async fn discover_poster(
        &self,
        media_type: Option<MediaType>,
        storefront: Option<StorefrontId>,
        external_id: Option<&str>,
        title: &str,
    ) -> Result<Option<DiscoveredAsset>, AssetError> {
        let opts = PosterSearchOptions::SteamGridDb(GridSearchOptions::poster(1, 1));
        Ok(self
            .search_poster(media_type, storefront, external_id, title, &opts)
            .await?
            .into_iter()
            .next())
    }

    async fn search_poster(
        &self,
        _media_type: Option<MediaType>,
        storefront: Option<StorefrontId>,
        external_id: Option<&str>,
        title: &str,
        options: &PosterSearchOptions,
    ) -> Result<Vec<DiscoveredAsset>, AssetError> {
        let grid_options = match options {
            PosterSearchOptions::SteamGridDb(opts) => opts.clone(),
            _ => GridSearchOptions::poster(1, 10),
        };

        let Some(game_id) = self
            .client
            .find_game_id(storefront, external_id, title)
            .await?
        else {
            return Ok(Vec::new());
        };

        let response = self.client.grids(game_id, grid_options).await?;

        Ok(response.into_assets_with_game_id(AssetType::Poster, Some(game_id)))
    }
}

#[async_trait]
impl BannerFetcher for SteamGridDb {
    async fn discover_banner(
        &self,
        media_type: Option<MediaType>,
        storefront: Option<StorefrontId>,
        external_id: Option<&str>,
        title: &str,
    ) -> Result<Option<DiscoveredAsset>, AssetError> {
        let opts = PosterSearchOptions::SteamGridDb(GridSearchOptions::hero(1, 1));
        Ok(self
            .search_banner(media_type, storefront, external_id, title, &opts)
            .await?
            .into_iter()
            .next())
    }

    async fn search_banner(
        &self,
        _media_type: Option<MediaType>,
        storefront: Option<StorefrontId>,
        external_id: Option<&str>,
        title: &str,
        options: &PosterSearchOptions,
    ) -> Result<Vec<DiscoveredAsset>, AssetError> {
        let grid_options = match options {
            PosterSearchOptions::SteamGridDb(opts) => opts.clone(),
            _ => GridSearchOptions::hero(1, 10),
        };

        let Some(game_id) = self
            .client
            .find_game_id(storefront, external_id, title)
            .await?
        else {
            return Ok(Vec::new());
        };

        let response = self.client.heroes(game_id, grid_options).await?;

        Ok(response.into_assets_with_game_id(AssetType::Banner, Some(game_id)))
    }
}
