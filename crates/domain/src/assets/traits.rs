use async_trait::async_trait;

use crate::{
    assets::error::AssetError,
    media::models::{AssetType, DiscoveredAsset},
    storefronts::models::StorefrontId,
};

#[async_trait]
pub trait AssetFetcher: Send + Sync {
    fn asset_type(&self) -> AssetType;
}

#[async_trait]
pub trait PosterFetcher: AssetFetcher {
    type SearchOptions;

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
        options: Self::SearchOptions,
    ) -> Result<Vec<DiscoveredAsset>, AssetError>;
}
