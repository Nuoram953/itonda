use std::{collections::HashMap, sync::Arc};

use crate::{
    assets::{
        error::AssetError,
        models::{AssetStoreId, PosterSearchOptions},
        traits::{BannerFetcher, PosterFetcher},
    },
    media::{discovered::DiscoveredAsset, types::MediaType},
    storefronts::models::StorefrontId,
};

#[derive(Clone, Default)]
pub struct AssetRegistry {
    posters: Vec<Arc<dyn PosterFetcher>>,
    banners: Vec<Arc<dyn BannerFetcher>>,
}

impl AssetRegistry {
    pub fn new() -> Self {
        Self {
            posters: Vec::new(),
            banners: Vec::new(),
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

    pub fn register_banner(&mut self, provider: Arc<dyn BannerFetcher>) {
        self.banners.push(provider);
    }

    pub fn banners(&self) -> &[Arc<dyn BannerFetcher>] {
        &self.banners
    }

    pub fn banner_providers(&self) -> Vec<Arc<dyn BannerFetcher>> {
        self.banners.clone()
    }

    pub fn get_banner(&self, id: AssetStoreId) -> Option<Arc<dyn BannerFetcher>> {
        self.banners.iter().find(|f| f.id() == id).cloned()
    }

    pub fn needs_discovery(
        &self,
        media_type: &MediaType,
        existing_counts: &HashMap<i64, usize>,
        limit: usize,
    ) -> bool {
        for poster in &self.posters {
            if poster.supports_media_type(media_type.clone()) {
                for asset_type in poster.discovered_asset_types() {
                    let count = existing_counts.get(&asset_type.id()).copied().unwrap_or(0);
                    if count < limit {
                        return true;
                    }
                }
            }
        }

        for banner in &self.banners {
            if banner.supports_media_type(media_type.clone()) {
                for asset_type in banner.discovered_asset_types() {
                    let count = existing_counts.get(&asset_type.id()).copied().unwrap_or(0);
                    if count < limit {
                        return true;
                    }
                }
            }
        }

        false
    }

    pub async fn discover(
        &self,
        media_type: MediaType,
        storefront: Option<StorefrontId>,
        external_id: Option<&str>,
        title: &str,
    ) -> Result<Vec<DiscoveredAsset>, AssetError> {
        self.discover_needed(
            media_type,
            storefront,
            external_id,
            title,
            &HashMap::new(),
            None,
        )
        .await
    }

    pub async fn discover_needed(
        &self,
        media_type: MediaType,
        storefront: Option<StorefrontId>,
        external_id: Option<&str>,
        title: &str,
        existing_counts: &HashMap<i64, usize>,
        limit: Option<usize>,
    ) -> Result<Vec<DiscoveredAsset>, AssetError> {
        let mut results = Vec::new();
        for poster in &self.posters {
            if poster.supports_media_type(media_type.clone()) {
                let needed = match limit {
                    Some(max) => poster.discovered_asset_types().iter().any(|asset_type| {
                        existing_counts.get(&asset_type.id()).copied().unwrap_or(0) < max
                    }),
                    None => true,
                };

                if needed
                    && let Some(asset) = poster
                        .discover_poster(Some(media_type.clone()), storefront, external_id, title)
                        .await?
                {
                    results.push(asset);
                }
            }
        }
        for banner in &self.banners {
            if banner.supports_media_type(media_type.clone()) {
                let needed = match limit {
                    Some(max) => banner.discovered_asset_types().iter().any(|asset_type| {
                        existing_counts.get(&asset_type.id()).copied().unwrap_or(0) < max
                    }),
                    None => true,
                };

                if needed
                    && let Some(asset) = banner
                        .discover_banner(Some(media_type.clone()), storefront, external_id, title)
                        .await?
                {
                    results.push(asset);
                }
            }
        }
        Ok(results)
    }

    pub async fn search_poster(
        &self,
        store_id: AssetStoreId,
        media_type: Option<MediaType>,
        storefront: Option<StorefrontId>,
        external_id: Option<&str>,
        title: &str,
        options: &PosterSearchOptions,
    ) -> Result<Vec<DiscoveredAsset>, AssetError> {
        if let Some(fetcher) = self.get_poster(store_id) {
            fetcher
                .search_poster(media_type, storefront, external_id, title, options)
                .await
        } else {
            Ok(Vec::new())
        }
    }

    pub async fn search_banner(
        &self,
        store_id: AssetStoreId,
        media_type: Option<MediaType>,
        storefront: Option<StorefrontId>,
        external_id: Option<&str>,
        title: &str,
        options: &PosterSearchOptions,
    ) -> Result<Vec<DiscoveredAsset>, AssetError> {
        if let Some(fetcher) = self.get_banner(store_id) {
            fetcher
                .search_banner(media_type, storefront, external_id, title, options)
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

        fn supports_media_type(&self, media_type: MediaType) -> bool {
            matches!(media_type, MediaType::Game)
        }
    }

    #[async_trait]
    impl PosterFetcher for DummyGamePosterFetcher {
        async fn discover_poster(
            &self,
            _media_type: Option<MediaType>,
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
            _media_type: Option<MediaType>,
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
                Some(MediaType::Game),
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

    #[test]
    fn checks_if_discovery_is_needed() {
        let mut registry = AssetRegistry::new();
        registry.register_poster(Arc::new(DummyGamePosterFetcher));

        let mut existing = HashMap::new();
        assert!(registry.needs_discovery(&MediaType::Game, &existing, 1));

        existing.insert(AssetType::Poster.id(), 1);
        assert!(!registry.needs_discovery(&MediaType::Game, &existing, 1));

        assert!(registry.needs_discovery(&MediaType::Game, &existing, 2));
        assert!(!registry.needs_discovery(&MediaType::Movie, &existing, 1));
    }

    struct MultiAssetFetcher;

    impl AssetFetcher for MultiAssetFetcher {
        fn id(&self) -> AssetStoreId {
            AssetStoreId::SteamGridDb
        }

        fn supports_media_type(&self, media_type: MediaType) -> bool {
            matches!(media_type, MediaType::Game)
        }
    }

    #[async_trait]
    impl PosterFetcher for MultiAssetFetcher {
        fn discovered_asset_types(&self) -> Vec<AssetType> {
            vec![AssetType::Poster, AssetType::Backdrop]
        }
        async fn discover_poster(
            &self,
            _media_type: Option<MediaType>,
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
            _media_type: Option<MediaType>,
            _storefront: Option<StorefrontId>,
            _external_id: Option<&str>,
            _title: &str,
            _options: &PosterSearchOptions,
        ) -> Result<Vec<DiscoveredAsset>, AssetError> {
            Ok(vec![])
        }
    }

    #[test]
    fn checks_discovery_needed_for_multi_asset_fetcher() {
        let mut registry = AssetRegistry::new();
        registry.register_poster(Arc::new(MultiAssetFetcher));

        let mut existing = HashMap::new();
        existing.insert(AssetType::Poster.id(), 1);

        assert!(registry.needs_discovery(&MediaType::Game, &existing, 1));

        existing.insert(AssetType::Backdrop.id(), 1);
        assert!(!registry.needs_discovery(&MediaType::Game, &existing, 1));
    }
}
