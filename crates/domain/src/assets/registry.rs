use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use crate::{
    assets::{
        error::AssetError,
        models::{AssetStoreId, DiscoverOptions, PosterSearchOptions},
        traits::{BannerFetcher, PillarScreenshotFetcher, PosterFetcher, ScreenshotFetcher},
        types::AssetType,
    },
    media::{discovered::DiscoveredAsset, types::MediaType},
    storefronts::models::StorefrontId,
};

#[derive(Clone, Default)]
pub struct AssetRegistry {
    posters: Vec<Arc<dyn PosterFetcher>>,
    banners: Vec<Arc<dyn BannerFetcher>>,
    screenshots: Vec<Arc<dyn ScreenshotFetcher>>,
    pillar_screenshots: Vec<Arc<dyn PillarScreenshotFetcher>>,
}

impl AssetRegistry {
    pub fn new() -> Self {
        Self {
            posters: Vec::new(),
            banners: Vec::new(),
            screenshots: Vec::new(),
            pillar_screenshots: Vec::new(),
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

    pub fn register_screenshot(&mut self, provider: Arc<dyn ScreenshotFetcher>) {
        self.screenshots.push(provider);
    }

    pub fn screenshots(&self) -> &[Arc<dyn ScreenshotFetcher>] {
        &self.screenshots
    }

    pub fn screenshot_providers(&self) -> Vec<Arc<dyn ScreenshotFetcher>> {
        self.screenshots.clone()
    }

    pub fn get_screenshot(&self, id: AssetStoreId) -> Option<Arc<dyn ScreenshotFetcher>> {
        self.screenshots.iter().find(|f| f.id() == id).cloned()
    }

    pub fn register_pillar_screenshot(&mut self, provider: Arc<dyn PillarScreenshotFetcher>) {
        self.pillar_screenshots.push(provider);
    }

    pub fn pillar_screenshots(&self) -> &[Arc<dyn PillarScreenshotFetcher>] {
        &self.pillar_screenshots
    }

    pub fn pillar_screenshot_providers(&self) -> Vec<Arc<dyn PillarScreenshotFetcher>> {
        self.pillar_screenshots.clone()
    }

    pub fn get_pillar_screenshot(
        &self,
        id: AssetStoreId,
    ) -> Option<Arc<dyn PillarScreenshotFetcher>> {
        self.pillar_screenshots
            .iter()
            .find(|f| f.id() == id)
            .cloned()
    }

    pub async fn discover(
        &self,
        media_type: MediaType,
        storefront: Option<StorefrontId>,
        external_id: Option<&str>,
        title: &str,
    ) -> Result<Vec<DiscoveredAsset>, AssetError> {
        let (assets, _) = self
            .discover_needed(
                media_type,
                storefront,
                external_id,
                title,
                DiscoverOptions {
                    existing_counts: &HashMap::new(),
                    searched_types: &HashSet::new(),
                    limit: None,
                    force: true,
                    external_ids: &[],
                    pillars: &[],
                },
            )
            .await?;
        Ok(assets)
    }

    pub async fn discover_needed(
        &self,
        media_type: MediaType,
        storefront: Option<StorefrontId>,
        external_id: Option<&str>,
        title: &str,
        options: DiscoverOptions<'_>,
    ) -> Result<(Vec<DiscoveredAsset>, HashSet<i64>), AssetError> {
        let mut results = Vec::new();
        let mut attempted = HashSet::new();

        for poster in &self.posters {
            if poster.supports_media_type(media_type) {
                let asset_types = poster.discovered_asset_types();
                let needed = options.force
                    || match options.limit {
                        Some(max) => asset_types.iter().any(|asset_type| {
                            !options.searched_types.contains(&asset_type.id())
                                && options
                                    .existing_counts
                                    .get(&asset_type.id())
                                    .copied()
                                    .unwrap_or(0)
                                    < max
                        }),
                        None => true,
                    };

                if needed {
                    for at in &asset_types {
                        attempted.insert(at.id());
                    }
                    if let Some(asset) = poster
                        .discover_poster(Some(media_type), storefront, external_id, title)
                        .await?
                    {
                        results.push(asset);
                    }
                }
            }
        }

        for banner in &self.banners {
            if banner.supports_media_type(media_type) {
                let asset_types = banner.discovered_asset_types();
                let needed = options.force
                    || match options.limit {
                        Some(max) => asset_types.iter().any(|asset_type| {
                            !options.searched_types.contains(&asset_type.id())
                                && options
                                    .existing_counts
                                    .get(&asset_type.id())
                                    .copied()
                                    .unwrap_or(0)
                                    < max
                        }),
                        None => true,
                    };

                if needed {
                    for at in &asset_types {
                        attempted.insert(at.id());
                    }
                    if let Some(asset) = banner
                        .discover_banner(Some(media_type), storefront, external_id, title)
                        .await?
                    {
                        results.push(asset);
                    }
                }
            }
        }

        for screenshot in &self.screenshots {
            if screenshot.supports_media_type(media_type) {
                let asset_types = screenshot.discovered_asset_types();
                let needed = options.force
                    || match options.limit {
                        Some(max) => asset_types.iter().any(|asset_type| {
                            !options.searched_types.contains(&asset_type.id())
                                && options
                                    .existing_counts
                                    .get(&asset_type.id())
                                    .copied()
                                    .unwrap_or(0)
                                    < max
                        }),
                        None => true,
                    };

                if needed {
                    for at in &asset_types {
                        attempted.insert(at.id());
                    }
                    if let Some(asset) = screenshot
                        .discover_screenshot(Some(media_type), storefront, external_id, title)
                        .await?
                    {
                        results.push(asset);
                    }
                }
            }
        }

        if !options.pillars.is_empty() {
            for pillar in options.pillars {
                if pillar.asset_id.is_none() || options.force {
                    for fetcher in &self.pillar_screenshots {
                        if fetcher.supports_media_type(media_type) {
                            if let Some(mut asset) = fetcher
                                .discover_pillar_screenshot(
                                    Some(media_type),
                                    storefront,
                                    external_id,
                                    title,
                                    &pillar.title,
                                )
                                .await?
                            {
                                asset.asset_type = AssetType::Screenshot;
                                asset.pillar_id = Some(pillar.id.clone());
                                results.push(asset);
                                break;
                            }
                        }
                    }
                }
            }
        }

        Ok((results, attempted))
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
            Ok(Some(DiscoveredAsset::new(
                AssetType::Poster,
                "http://example.com/poster.png",
            )))
        }

        async fn search_poster(
            &self,
            _media_type: Option<MediaType>,
            _storefront: Option<StorefrontId>,
            _external_id: Option<&str>,
            _title: &str,
            _options: &PosterSearchOptions,
        ) -> Result<Vec<DiscoveredAsset>, AssetError> {
            Ok(vec![DiscoveredAsset::new(
                AssetType::Poster,
                "http://example.com/poster.png",
            )])
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

    #[tokio::test]
    async fn checks_if_discovery_is_needed() {
        let mut registry = AssetRegistry::new();
        registry.register_poster(Arc::new(DummyGamePosterFetcher));

        let mut existing = HashMap::new();
        let searched = HashSet::new();

        let (discovered, attempted) = registry
            .discover_needed(
                MediaType::Game,
                Some(StorefrontId::Steam),
                Some("123"),
                "Test",
                DiscoverOptions {
                    existing_counts: &existing,
                    searched_types: &searched,
                    limit: Some(1),
                    force: false,
                    external_ids: &[],
                    pillars: &[],
                },
            )
            .await
            .unwrap();
        assert_eq!(discovered.len(), 1);
        assert!(attempted.contains(&AssetType::Poster.id()));

        existing.insert(AssetType::Poster.id(), 1);
        let (discovered, attempted) = registry
            .discover_needed(
                MediaType::Game,
                Some(StorefrontId::Steam),
                Some("123"),
                "Test",
                DiscoverOptions {
                    existing_counts: &existing,
                    searched_types: &searched,
                    limit: Some(1),
                    force: false,
                    external_ids: &[],
                    pillars: &[],
                },
            )
            .await
            .unwrap();
        assert_eq!(discovered.len(), 0);
        assert!(attempted.is_empty());

        let (discovered, attempted) = registry
            .discover_needed(
                MediaType::Game,
                Some(StorefrontId::Steam),
                Some("123"),
                "Test",
                DiscoverOptions {
                    existing_counts: &existing,
                    searched_types: &searched,
                    limit: Some(2),
                    force: false,
                    external_ids: &[],
                    pillars: &[],
                },
            )
            .await
            .unwrap();
        assert_eq!(discovered.len(), 1);
        assert!(attempted.contains(&AssetType::Poster.id()));

        let (discovered, _) = registry
            .discover_needed(
                MediaType::Movie,
                Some(StorefrontId::Steam),
                Some("123"),
                "Test",
                DiscoverOptions {
                    existing_counts: &existing,
                    searched_types: &searched,
                    limit: Some(1),
                    force: false,
                    external_ids: &[],
                    pillars: &[],
                },
            )
            .await
            .unwrap();
        assert_eq!(discovered.len(), 0);
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
            Ok(Some(DiscoveredAsset::new(
                AssetType::Poster,
                "http://example.com/poster.png",
            )))
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

    #[tokio::test]
    async fn checks_discovery_needed_for_multi_asset_fetcher() {
        let mut registry = AssetRegistry::new();
        registry.register_poster(Arc::new(MultiAssetFetcher));

        let mut existing = HashMap::new();
        let searched = HashSet::new();
        existing.insert(AssetType::Poster.id(), 1);

        let (discovered, _) = registry
            .discover_needed(
                MediaType::Game,
                Some(StorefrontId::Steam),
                Some("123"),
                "Test",
                DiscoverOptions {
                    existing_counts: &existing,
                    searched_types: &searched,
                    limit: Some(1),
                    force: false,
                    external_ids: &[],
                    pillars: &[],
                },
            )
            .await
            .unwrap();
        assert_eq!(discovered.len(), 1);

        existing.insert(AssetType::Backdrop.id(), 1);
        let (discovered, _) = registry
            .discover_needed(
                MediaType::Game,
                Some(StorefrontId::Steam),
                Some("123"),
                "Test",
                DiscoverOptions {
                    existing_counts: &existing,
                    searched_types: &searched,
                    limit: Some(1),
                    force: false,
                    external_ids: &[],
                    pillars: &[],
                },
            )
            .await
            .unwrap();
        assert_eq!(discovered.len(), 0);
    }

    struct DummyScreenshotFetcher;

    impl AssetFetcher for DummyScreenshotFetcher {
        fn id(&self) -> AssetStoreId {
            AssetStoreId::DuckDuckGo
        }

        fn supports_media_type(&self, media_type: MediaType) -> bool {
            matches!(media_type, MediaType::Game)
        }
    }

    #[async_trait]
    impl ScreenshotFetcher for DummyScreenshotFetcher {
        async fn discover_screenshot(
            &self,
            _media_type: Option<MediaType>,
            _storefront: Option<StorefrontId>,
            _external_id: Option<&str>,
            title: &str,
        ) -> Result<Option<DiscoveredAsset>, AssetError> {
            Ok(Some(DiscoveredAsset::new(
                AssetType::Screenshot,
                format!("http://example.com/{}_screenshot.jpg", title),
            )))
        }
    }

    struct DummyPillarScreenshotFetcher;

    impl AssetFetcher for DummyPillarScreenshotFetcher {
        fn id(&self) -> AssetStoreId {
            AssetStoreId::DuckDuckGo
        }

        fn supports_media_type(&self, media_type: MediaType) -> bool {
            matches!(media_type, MediaType::Game)
        }
    }

    #[async_trait]
    impl PillarScreenshotFetcher for DummyPillarScreenshotFetcher {
        async fn discover_pillar_screenshot(
            &self,
            _media_type: Option<MediaType>,
            _storefront: Option<StorefrontId>,
            _external_id: Option<&str>,
            game_title: &str,
            pillar_title: &str,
        ) -> Result<Option<DiscoveredAsset>, AssetError> {
            Ok(Some(DiscoveredAsset::new(
                AssetType::Screenshot,
                format!("http://example.com/{}_{}.jpg", game_title, pillar_title),
            )))
        }
    }

    #[tokio::test]
    async fn registers_and_discovers_screenshot() {
        let mut registry = AssetRegistry::new();
        registry.register_screenshot(Arc::new(DummyScreenshotFetcher));

        let assets = registry
            .discover(MediaType::Game, None, None, "Gears of War")
            .await
            .unwrap();

        assert_eq!(assets.len(), 1);
        assert_eq!(assets[0].asset_type, AssetType::Screenshot);
        assert_eq!(assets[0].url, "http://example.com/Gears of War_screenshot.jpg");
    }

    #[tokio::test]
    async fn discovers_screenshots_for_pillars_missing_asset_id() {
        use crate::media::models::GameplayPillar;

        let mut registry = AssetRegistry::new();
        registry.register_screenshot(Arc::new(DummyScreenshotFetcher));
        registry.register_pillar_screenshot(Arc::new(DummyPillarScreenshotFetcher));

        let pillars = vec![
            GameplayPillar {
                id: "active_reload".into(),
                title: "Active Reload".into(),
                description: "Reload mechanic".into(),
                icon: "combat".into(),
                asset_id: None,
            },
            GameplayPillar {
                id: "squad_tagging".into(),
                title: "Squad Tagging".into(),
                description: "Tagging mechanic".into(),
                icon: "coop".into(),
                asset_id: Some("existing_uuid".into()),
            },
        ];

        let (discovered, _) = registry
            .discover_needed(
                MediaType::Game,
                None,
                None,
                "Gears of War",
                DiscoverOptions {
                    existing_counts: &HashMap::new(),
                    searched_types: &HashSet::new(),
                    limit: None,
                    force: false,
                    external_ids: &[],
                    pillars: &pillars,
                },
            )
            .await
            .unwrap();

        let pillar_shots: Vec<_> = discovered
            .iter()
            .filter(|a| a.pillar_id.is_some())
            .collect();
        assert_eq!(pillar_shots.len(), 1);
        assert_eq!(pillar_shots[0].asset_type, AssetType::Screenshot);
        assert_eq!(pillar_shots[0].pillar_id.as_deref(), Some("active_reload"));
        assert_eq!(
            pillar_shots[0].url,
            "http://example.com/Gears of War_Active Reload.jpg"
        );

        let regular_shots: Vec<_> = discovered
            .iter()
            .filter(|a| a.pillar_id.is_none())
            .collect();
        assert_eq!(regular_shots.len(), 1);
        assert_eq!(regular_shots[0].asset_type, AssetType::Screenshot);
        assert_eq!(
            regular_shots[0].url,
            "http://example.com/Gears of War_screenshot.jpg"
        );
    }
}
