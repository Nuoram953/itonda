use async_trait::async_trait;

use crate::{
    media::types::MediaType,
    metadata::{
        error::MetadataError,
        models::{GeneralMetadata, MetadataProviderId, MetadataQuery},
        traits::{GeneralInfoFetcher, MetadataFetcher},
    },
    sources::the_internet_game_database::client::TheInternetGameDatabaseClient,
};

pub mod client;
pub mod models;

#[cfg(test)]
pub mod tests;

pub struct TheInternetGameDatabase {
    client: TheInternetGameDatabaseClient,
}

impl TheInternetGameDatabase {
    pub fn new(client_id: String, client_secret: String) -> Self {
        Self {
            client: TheInternetGameDatabaseClient::new(client_id, client_secret),
        }
    }

    pub fn client(&self) -> &TheInternetGameDatabaseClient {
        &self.client
    }
}

impl MetadataFetcher for TheInternetGameDatabase {
    fn id(&self) -> MetadataProviderId {
        MetadataProviderId::TheInternetGameDatabase
    }

    fn name(&self) -> &'static str {
        "IGDB"
    }

    fn supports_media_type(&self, media_type: MediaType) -> bool {
        matches!(media_type, MediaType::Game)
    }
}

#[async_trait]
impl GeneralInfoFetcher for TheInternetGameDatabase {
    async fn fetch_general_info(
        &self,
        query: &MetadataQuery<'_>,
    ) -> Result<Option<GeneralMetadata>, MetadataError> {
        let Some(game_id) = self
            .client
            .find_game_id(query.storefront, query.external_id, query.title)
            .await?
        else {
            return Ok(None);
        };

        let Some(game) = self.client.get_game(game_id).await? else {
            return Ok(None);
        };

        Ok(Some(game.into_general_metadata()))
    }
}
