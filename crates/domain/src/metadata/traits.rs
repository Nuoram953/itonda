use async_trait::async_trait;

use crate::{
    media::types::MediaType,
    metadata::{
        error::MetadataError,
        models::{
            GeneralMetadata, MediaSearchResult, MetadataProviderId, MetadataQuery, MetadataType,
        },
    },
};

#[async_trait]
pub trait MetadataFetcher: Send + Sync {
    fn id(&self) -> MetadataProviderId;
    fn name(&self) -> &'static str;
    fn metadata_type(&self) -> MetadataType {
        MetadataType::General
    }
    fn supports_media_type(&self, media_type: MediaType) -> bool {
        self.metadata_type().supports_media_type(media_type)
    }
}

#[async_trait]
pub trait GeneralInfoFetcher: MetadataFetcher {
    async fn fetch_general_info(
        &self,
        query: &MetadataQuery<'_>,
    ) -> Result<Option<GeneralMetadata>, MetadataError>;
}

#[async_trait]
pub trait MediaSearcher: Send + Sync {
    fn supports_media_type(&self, media_type: MediaType) -> bool;
    async fn search(
        &self,
        query: &str,
        media_type: MediaType,
    ) -> Result<Vec<MediaSearchResult>, MetadataError>;
}
