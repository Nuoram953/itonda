use reqwest_middleware::ClientWithMiddleware;

use crate::{
    http::create_http_client,
    storefronts::{error::StorefrontError, steam::models::GetOwnedGamesResponse},
};

pub struct SteamClient {
    client: ClientWithMiddleware,
    api_key: String,
    base_url: String,
}

impl SteamClient {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            client: create_http_client(),
            api_key: api_key.into(),
            base_url: "https://api.steampowered.com".into(),
        }
    }

    pub async fn get_owned_games(
        &self,
        steam_id: &str,
    ) -> Result<GetOwnedGamesResponse, StorefrontError> {
        let response = self
            .client
            .get(format!(
                "{}/IPlayerService/GetOwnedGames/v1/",
                self.base_url
            ))
            .query(&[
                ("key", self.api_key.as_str()),
                ("steamid", steam_id),
                ("include_appinfo", "true"),
            ])
            .send()
            .await?
            .json::<GetOwnedGamesResponse>()
            .await?;

        Ok(response)
    }
}
