use async_trait::async_trait;

use crate::{
    media::types::MediaType,
    metadata::{
        error::MetadataError,
        models::{GeneralMetadata, MetadataProviderId, MetadataQuery},
    },
};

#[async_trait]
pub trait MetadataFetcher: Send + Sync {
    fn id(&self) -> MetadataProviderId;
    fn name(&self) -> &'static str;
    fn supports_media_type(&self, _media_type: MediaType) -> bool {
        true
    }
}

#[async_trait]
pub trait GeneralInfoFetcher: MetadataFetcher {
    async fn fetch_general_info(
        &self,
        query: &MetadataQuery<'_>,
    ) -> Result<Option<GeneralMetadata>, MetadataError>;
}
