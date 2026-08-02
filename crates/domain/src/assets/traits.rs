use async_trait::async_trait;

use crate::{
    assets::{
        error::AssetError,
        models::{AssetStoreId, PosterSearchOptions},
        types::AssetType,
    },
    media::{discovered::DiscoveredAsset, types::MediaType},
    storefronts::models::StorefrontId,
};

#[async_trait]
pub trait AssetFetcher: Send + Sync {
    fn id(&self) -> AssetStoreId;
    fn asset_type(&self) -> AssetType;
    fn supports_media_type(&self, _media_type: MediaType) -> bool {
        true
    }
}

#[async_trait]
pub trait PosterFetcher: AssetFetcher {
    async fn discover_poster(
        &self,
        storefront: StorefrontId,
        external_id: Option<&str>,
        title: &str,
    ) -> Result<Option<DiscoveredAsset>, AssetError>;

    async fn search_poster(
        &self,
        storefront: StorefrontId,
        external_id: Option<&str>,
        title: &str,
        options: &PosterSearchOptions,
    ) -> Result<Vec<DiscoveredAsset>, AssetError>;
}
