use std::time::{Duration, Instant};

use reqwest_middleware::ClientWithMiddleware;
use tokio::sync::RwLock;

use crate::{
    http::{RateLimiter, create_rate_limited_http_client},
    metadata::error::MetadataError,
    sources::the_internet_game_database::models::{
        CachedToken, GetExternalGameResponse, GetGameResponse, GetInvolvedCompanyResponse,
        GetSearchResponse, TwitchTokenResponse,
    },
    storefronts::models::StorefrontId,
};

pub struct TheInternetGameDatabaseClient {
    client: ClientWithMiddleware,
    client_id: String,
    client_secret: String,
    base_url: String,
    pub(crate) token_cache: RwLock<Option<CachedToken>>,
}

impl TheInternetGameDatabaseClient {
    pub fn new(client_id: impl Into<String>, client_secret: impl Into<String>) -> Self {
        Self::with_rate_limiter(client_id, client_secret, RateLimiter::new(6, 2.0))
    }

    pub fn with_rate_limiter(
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
        rate_limiter: RateLimiter,
    ) -> Self {
        Self {
            client: create_rate_limited_http_client(rate_limiter),
            client_id: client_id.into(),
            client_secret: client_secret.into(),
            base_url: "https://api.igdb.com/v4/".into(),
            token_cache: RwLock::new(None),
        }
    }

    #[cfg(test)]
    pub fn with_base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into();
        self
    }

    pub async fn get_token(&self) -> Result<String, MetadataError> {
        let now = Instant::now();

        {
            let read_guard = self.token_cache.read().await;
            if let Some(cached) = read_guard.as_ref()
                && cached.expires_at > now
            {
                return Ok(cached.token.clone());
            }
        }

        let mut write_guard = self.token_cache.write().await;

        if let Some(cached) = write_guard.as_ref()
            && cached.expires_at > now
        {
            return Ok(cached.token.clone());
        }

        let response = self
            .client
            .post("https://id.twitch.tv/oauth2/token")
            .query(&[
                ("client_id", self.client_id.as_str()),
                ("client_secret", self.client_secret.as_str()),
                ("grant_type", "client_credentials"),
            ])
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(MetadataError::Authentication);
        }

        let token_data = response
            .json::<TwitchTokenResponse>()
            .await
            .map_err(MetadataError::Http)?;

        let valid_duration = Duration::from_secs(token_data.expires_in.saturating_sub(60));
        let expires_at = Instant::now() + valid_duration;

        let token = token_data.access_token.clone();
        *write_guard = Some(CachedToken {
            token: token_data.access_token,
            expires_at,
        });

        Ok(token)
    }

    async fn post_igdb(
        &self,
        endpoint: &str,
        body: String,
    ) -> Result<reqwest::Response, MetadataError> {
        let token = self.get_token().await?;

        let url = format!("{}{}", self.base_url, endpoint);
        let response = self
            .client
            .post(&url)
            .bearer_auth(&token)
            .header("Client-ID", &self.client_id)
            .header("Accept", "application/json")
            .body(body)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(MetadataError::Other(format!(
                "IGDB {} error ({}): {}",
                endpoint, status, error_text
            )));
        }

        Ok(response)
    }

    pub async fn get_external_game_id(
        &self,
        external_id: &str,
        external_game_source: i32,
    ) -> Result<Option<u64>, MetadataError> {
        let response = self
            .post_igdb(
                "external_games",
                format!(
                    "fields game; where uid = \"{}\" & external_game_source = {};",
                    external_id, external_game_source
                ),
            )
            .await?;

        let response = response.json::<Vec<GetExternalGameResponse>>().await?;

        Ok(response.into_iter().next().map(|item| item.game))
    }

    pub async fn search(&self, title: &str) -> Result<Option<u64>, MetadataError> {
        let sanitized_title = title.replace('"', "\\\"");
        let response = self
            .post_igdb(
                "games",
                format!("fields id; search \"{}\"; limit 1;", sanitized_title),
            )
            .await?;

        let response = response.json::<Vec<GetSearchResponse>>().await?;

        Ok(response.into_iter().next().map(|item| item.id))
    }

    pub async fn find_game_id(
        &self,
        storefront: Option<StorefrontId>,
        external_id: Option<&str>,
        title: &str,
    ) -> Result<Option<u64>, MetadataError> {
        if let (Some(storefront), Some(external_id)) = (storefront, external_id) {
            let external_source = storefront.as_the_internet_game_database();
            if let Some(id) = self
                .get_external_game_id(external_id, external_source)
                .await?
            {
                return Ok(Some(id));
            }
        }

        self.search(title).await
    }

    pub async fn get_game(&self, id: u64) -> Result<Option<GetGameResponse>, MetadataError> {
        let response = self
            .post_igdb(
                "games",
                format!(
                    "fields id, name, summary, storyline, first_release_date, url, genres.name, platforms.name, themes.name, game_modes.name, collections.name, franchises.name, screenshots.url, involved_companies.developer, involved_companies.publisher, involved_companies.company.name; where id = {};",
                    id
                ),
            )
            .await?;

        let mut response = response.json::<Vec<GetGameResponse>>().await?;

        Ok(response.pop())
    }

    pub async fn get_involved_companies(
        &self,
        id: u64,
    ) -> Result<Vec<GetInvolvedCompanyResponse>, MetadataError> {
        let response = self
            .post_igdb(
                "involved_companies",
                format!("fields *, company.name; where game = {};", id),
            )
            .await?;

        let response = response.json::<Vec<GetInvolvedCompanyResponse>>().await?;

        Ok(response)
    }
}
