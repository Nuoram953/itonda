use reqwest_middleware::ClientWithMiddleware;

use crate::{
    assets::error::AssetError,
    http::create_http_client,
    sources::steam_grid_db::models::{
        GetExternalGameIdResponse, GridSearchOptions, MediaResponse, SearchResponse,
    },
    storefronts::models::StorefrontId,
};

pub struct SteamGridDbClient {
    client: ClientWithMiddleware,
    api_key: String,
    base_url: String,
}

impl SteamGridDbClient {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            client: create_http_client(),
            api_key: api_key.into(),
            base_url: "https://www.steamgriddb.com/api/v2/".into(),
        }
    }

    pub async fn find_game_id(
        &self,
        storefront: Option<StorefrontId>,
        external_id: Option<&str>,
        title: &str,
    ) -> Result<Option<u32>, AssetError> {
        if let Some(external_id) = external_id
            && let Some(storefront) = storefront
            && let Some(id) = self
                .find_game_by_external_id(storefront, external_id)
                .await?
        {
            return Ok(Some(id));
        }

        self.search_game(title).await
    }

    pub async fn find_game_by_external_id(
        &self,
        storefront: StorefrontId,
        external_id: &str,
    ) -> Result<Option<u32>, AssetError> {
        let platform = storefront.as_steam_grid_db_platform();

        let response = self
            .client
            .get(format!(
                "{}games/{}/{}",
                self.base_url, platform, external_id
            ))
            .bearer_auth(&self.api_key)
            .send()
            .await?;

        if response.status() == reqwest::StatusCode::NOT_FOUND {
            return Ok(None);
        }

        let response = response.json::<GetExternalGameIdResponse>().await?;

        Ok(Some(response.data.id))
    }

    pub async fn search_game(&self, title: &str) -> Result<Option<u32>, AssetError> {
        let response = self
            .client
            .get(format!("{}search/autocomplete/{}", self.base_url, title))
            .bearer_auth(&self.api_key)
            .send()
            .await?;

        let response = response.json::<SearchResponse>().await?;

        Ok(response.data.first().map(|game| game.id))
    }

    pub async fn grids(
        &self,
        game_id: u32,
        options: GridSearchOptions,
    ) -> Result<MediaResponse, AssetError> {
        let response = self
            .client
            .get(format!("{}grids/game/{}", self.base_url, game_id))
            .bearer_auth(&self.api_key)
            .query(&options)
            .send()
            .await?;

        let response = response.json::<MediaResponse>().await?;

        Ok(response)
    }

    pub async fn heroes(
        &self,
        game_id: u32,
        options: GridSearchOptions,
    ) -> Result<MediaResponse, AssetError> {
        let response = self
            .client
            .get(format!("{}heroes/game/{}", self.base_url, game_id))
            .bearer_auth(&self.api_key)
            .query(&options)
            .send()
            .await?;

        let response = response.json::<MediaResponse>().await?;

        Ok(response)
    }
}
