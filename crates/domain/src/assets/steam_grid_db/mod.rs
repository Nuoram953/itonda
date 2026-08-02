use async_trait::async_trait;

use crate::{
    assets::{
        error::AssetError,
        models::{AssetStoreId, PosterSearchOptions},
        steam_grid_db::{client::SteamGridDbClient, models::GridSearchOptions},
        traits::{AssetFetcher, PosterFetcher},
        types::AssetType,
    },
    media::{discovered::DiscoveredAsset, types::MediaType},
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

    fn asset_type(&self) -> AssetType {
        AssetType::Poster
    }

    fn supports_media_type(&self, media_type: MediaType) -> bool {
        matches!(media_type, MediaType::Game)
    }
}

#[async_trait]
impl PosterFetcher for SteamGridDb {
    async fn discover_poster(
        &self,
        storefront: StorefrontId,
        external_id: Option<&str>,
        title: &str,
    ) -> Result<Option<DiscoveredAsset>, AssetError> {
        let opts = PosterSearchOptions::SteamGridDb(GridSearchOptions::poster(1, 1));
        Ok(self
            .search_poster(storefront, external_id, title, &opts)
            .await?
            .into_iter()
            .next())
    }

    async fn search_poster(
        &self,
        storefront: StorefrontId,
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

        Ok(response.into_assets(AssetType::Poster))
    }
}
