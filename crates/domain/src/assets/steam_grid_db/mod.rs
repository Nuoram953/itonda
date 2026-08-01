use async_trait::async_trait;

use crate::{
    assets::{
        error::AssetError,
        steam_grid_db::{client::SteamGridDbClient, models::GridSearchOptions},
        traits::{AssetFetcher, PosterFetcher},
    },
    media::models::{AssetType, DiscoveredAsset},
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
    fn asset_type(&self) -> AssetType {
        todo!()
    }
}

#[async_trait]
impl PosterFetcher for SteamGridDb {
    type SearchOptions = GridSearchOptions;

    async fn discover_poster(
        &self,
        storefront: StorefrontId,
        external_id: Option<&str>,
        title: &str,
    ) -> Result<Option<DiscoveredAsset>, AssetError> {
        Ok(self
            .search_poster(
                storefront,
                external_id,
                title,
                GridSearchOptions::poster(1, 1),
            )
            .await?
            .into_iter()
            .next())
    }

    async fn search_poster(
        &self,
        storefront: StorefrontId,
        external_id: Option<&str>,
        title: &str,
        options: GridSearchOptions,
    ) -> Result<Vec<DiscoveredAsset>, AssetError> {
        let Some(game_id) = self
            .client
            .find_game_id(storefront, external_id, title)
            .await?
        else {
            return Ok(Vec::new());
        };

        let response = self.client.grids(game_id, options).await?;

        Ok(response.into_assets(AssetType::Poster))
    }
}
