use std::sync::Arc;

use crate::{
    assets::{
        error::AssetError,
        models::{AssetStoreId, PosterSearchOptions},
        traits::PosterFetcher,
    },
    media::{discovered::DiscoveredAsset, types::MediaType},
    storefronts::models::StorefrontId,
};

#[derive(Clone, Default)]
pub struct AssetRegistry {
    posters: Vec<Arc<dyn PosterFetcher>>,
}

impl AssetRegistry {
    pub fn new() -> Self {
        Self {
            posters: Vec::new(),
        }
    }

    pub fn register_poster(&mut self, provider: Arc<dyn PosterFetcher>) {
        self.posters.push(provider);
    }

    pub fn posters(&self) -> &[Arc<dyn PosterFetcher>] {
        &self.posters
    }

    pub fn poster_providers(&self) -> Vec<Arc<dyn PosterFetcher>> {
        self.posters.clone()
    }

    pub fn get_poster(&self, id: AssetStoreId) -> Option<Arc<dyn PosterFetcher>> {
        self.posters.iter().find(|f| f.id() == id).cloned()
    }

    pub async fn discover(
        &self,
        media_type: MediaType,
        storefront: Option<StorefrontId>,
        external_id: Option<&str>,
        title: &str,
    ) -> Result<Vec<DiscoveredAsset>, AssetError> {
        let mut results = Vec::new();
        for poster in &self.posters {
            if poster.supports_media_type(media_type.clone())
                && let Some(asset) = poster
                    .discover_poster(storefront, external_id, title)
                    .await?
            {
                results.push(asset);
            }
        }
        Ok(results)
    }

    pub async fn search_poster(
        &self,
        store_id: AssetStoreId,
        storefront: Option<StorefrontId>,
        external_id: Option<&str>,
        title: &str,
        options: &PosterSearchOptions,
    ) -> Result<Vec<DiscoveredAsset>, AssetError> {
        if let Some(fetcher) = self.get_poster(store_id) {
            fetcher
                .search_poster(storefront, external_id, title, options)
                .await
        } else {
            Ok(Vec::new())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assets::{traits::AssetFetcher, types::AssetType};
    use async_trait::async_trait;

    struct DummyGamePosterFetcher;

    impl AssetFetcher for DummyGamePosterFetcher {
        fn id(&self) -> AssetStoreId {
            AssetStoreId::SteamGridDb
        }

        fn asset_type(&self) -> AssetType {
            AssetType::Poster
        }

        fn supports_media_type(&self, media_type: MediaType) -> bool {
            matches!(media_type, MediaType::Game)
        }
    }

    #[async_trait]
    impl PosterFetcher for DummyGamePosterFetcher {
        async fn discover_poster(
            &self,
            _storefront: Option<StorefrontId>,
            _external_id: Option<&str>,
            _title: &str,
        ) -> Result<Option<DiscoveredAsset>, AssetError> {
            Ok(Some(DiscoveredAsset {
                asset_type: AssetType::Poster,
                url: "http://example.com/poster.png".into(),
            }))
        }

        async fn search_poster(
            &self,
            _storefront: Option<StorefrontId>,
            _external_id: Option<&str>,
            _title: &str,
            _options: &PosterSearchOptions,
        ) -> Result<Vec<DiscoveredAsset>, AssetError> {
            Ok(vec![DiscoveredAsset {
                asset_type: AssetType::Poster,
                url: "http://example.com/poster.png".into(),
            }])
        }
    }

    #[tokio::test]
    async fn registers_and_discovers_posters() {
        let mut registry = AssetRegistry::new();
        registry.register_poster(Arc::new(DummyGamePosterFetcher));

        let posters = registry
            .discover(
                MediaType::Game,
                Some(StorefrontId::Steam),
                Some("123"),
                "Test",
            )
            .await
            .unwrap();

        assert_eq!(posters.len(), 1);
        assert_eq!(posters[0].url, "http://example.com/poster.png");
    }

    #[tokio::test]
    async fn ignores_providers_that_do_not_support_media_type() {
        let mut registry = AssetRegistry::new();
        registry.register_poster(Arc::new(DummyGamePosterFetcher));

        let posters = registry
            .discover(
                MediaType::Movie,
                Some(StorefrontId::Steam),
                Some("123"),
                "Test",
            )
            .await
            .unwrap();

        assert_eq!(posters.len(), 0);
    }

    #[tokio::test]
    async fn searches_poster_with_options() {
        let mut registry = AssetRegistry::new();
        registry.register_poster(Arc::new(DummyGamePosterFetcher));

        let posters = registry
            .search_poster(
                AssetStoreId::SteamGridDb,
                Some(StorefrontId::Steam),
                Some("123"),
                "Test",
                &PosterSearchOptions::Default,
            )
            .await
            .unwrap();

        assert_eq!(posters.len(), 1);
        assert_eq!(posters[0].url, "http://example.com/poster.png");
    }
}
