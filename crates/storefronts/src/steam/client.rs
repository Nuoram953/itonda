use itonda_domain::http::create_http_client;
use reqwest_middleware::ClientWithMiddleware;

use crate::{error::StorefrontError, steam::models::GetOwnedGamesResponse};

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
        steam_id: u64,
    ) -> Result<GetOwnedGamesResponse, StorefrontError> {
        let response = self
            .client
            .get(format!(
                "{}/IPlayerService/GetOwnedGames/v1/",
                self.base_url
            ))
            .query(&[
                ("key", self.api_key.as_str()),
                ("steamid", &steam_id.to_string()),
                ("include_appinfo", "true"),
            ])
            .send()
            .await?
            .json::<GetOwnedGamesResponse>()
            .await?;

        Ok(response)
    }
}
