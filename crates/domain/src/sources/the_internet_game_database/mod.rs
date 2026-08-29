use async_trait::async_trait;

use crate::{
    media::{
        models::{ExternalIdProvider, MediaExternalId},
        types::MediaType,
    },
    metadata::{
        error::MetadataError,
        models::{GeneralMetadata, MetadataProviderId, MetadataQuery},
        traits::{GeneralInfoFetcher, MetadataFetcher},
    },
    sources::the_internet_game_database::client::TheInternetGameDatabaseClient,
    storefronts::models::StorefrontId,
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
        let igdb_id = query
            .external_ids
            .iter()
            .find(|e| e.provider == ExternalIdProvider::Igdb)
            .and_then(|e| e.external_id.parse::<u64>().ok());

        let game_id = match igdb_id {
            Some(id) => Some(id),
            None => {
                let steam_id = query
                    .external_ids
                    .iter()
                    .find(|e| e.provider == ExternalIdProvider::Steam)
                    .map(|e| e.external_id.as_str())
                    .or(query.external_id);
                let storefront = query
                    .storefront
                    .or_else(|| steam_id.map(|_| StorefrontId::Steam));
                self.client
                    .find_game_id(storefront, steam_id, query.title)
                    .await?
            }
        };

        let Some(game_id) = game_id else {
            return Ok(None);
        };

        let Some(game) = self.client.get_game(game_id).await? else {
            return Ok(None);
        };

        let mut metadata = game.into_general_metadata();
        metadata.common_mut().external_ids.push(MediaExternalId {
            provider: ExternalIdProvider::Igdb,
            external_id: game_id.to_string(),
        });

        Ok(Some(metadata))
    }
}
