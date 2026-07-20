use crate::{
    error::StorefrontError,
    models::OwnedGame,
    steam::{client::SteamClient, mapper::map_owned_game},
    traits::GameLibraryProvider,
};
use async_trait::async_trait;

mod client;
mod mapper;
mod models;

#[cfg(test)]
mod tests;

pub struct SteamStorefront {
    client: SteamClient,
    steam_id: u64,
}

impl SteamStorefront {
    pub fn new(api_key: String, steam_id: u64) -> Self {
        Self {
            client: SteamClient::new(api_key),
            steam_id,
        }
    }
}

#[async_trait]
impl GameLibraryProvider for SteamStorefront {
    async fn owned_games(&self) -> Result<Vec<OwnedGame>, StorefrontError> {
        let response = self.client.get_owned_games(self.steam_id).await?;

        Ok(response
            .response
            .games
            .unwrap_or(Vec::new())
            .into_iter()
            .map(map_owned_game)
            .collect())
    }
}
