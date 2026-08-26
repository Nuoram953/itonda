use std::{
    collections::{HashMap, HashSet},
    slice::from_ref,
};

use async_trait::async_trait;
use sqlx::SqlitePool;

use itonda_database::media::{
    MediaAssetInsert, MediaAssetSearchInsert, find_asset_searches_by_media_ids,
    find_assets_by_media_ids, insert_media_asset, insert_media_asset_search,
};
use uuid::Uuid;

use crate::{
    assets::{
        downloader::AssetDownloader, models::DiscoverOptions, policy::AssetPolicy,
        registry::AssetRegistry,
    },
    sync::{context::SyncContext, errors::SyncError, pipeline::SyncStep},
};

pub struct AssetStep {
    pool: SqlitePool,
    registry: AssetRegistry,
    downloader: AssetDownloader,
    policy: AssetPolicy,
}

impl AssetStep {
    pub fn new(pool: SqlitePool, registry: AssetRegistry, downloader: AssetDownloader) -> Self {
        Self::with_policy(pool, registry, downloader, AssetPolicy::default())
    }

    pub fn with_policy(
        pool: SqlitePool,
        registry: AssetRegistry,
        downloader: AssetDownloader,
        policy: AssetPolicy,
    ) -> Self {
        Self {
            pool,
            registry,
            downloader,
            policy,
        }
    }

    pub fn policy(&self) -> AssetPolicy {
        self.policy
    }
}

#[async_trait]
impl SyncStep for AssetStep {
    fn name(&self) -> &'static str {
        "Assets"
    }

    async fn execute(&self, context: &mut SyncContext) -> Result<(), SyncError> {
        let media = context.media.as_ref().ok_or(SyncError::MissingMedia)?;

        let existing_assets = find_assets_by_media_ids(&self.pool, from_ref(&media.id)).await?;
        let mut existing_counts = HashMap::<i64, usize>::new();
        for asset in &existing_assets {
            *existing_counts.entry(asset.asset_id).or_default() += 1;
        }

        let existing_searches =
            find_asset_searches_by_media_ids(&self.pool, from_ref(&media.id)).await?;
        let searched_types: HashSet<i64> =
            existing_searches.into_iter().map(|s| s.asset_id).collect();

        let (media_type, storefront, external_id, title) = match &context.discovered {
            Some(discovered) => (
                discovered.media_type,
                Some(discovered.storefront),
                Some(discovered.external_id.as_str()),
                discovered.title.as_str(),
            ),
            None => (media.media_type, None, None, media.title.as_str()),
        };

        let limit = self.policy.max_items();
        let force = context.force;

        let (discovered_assets, attempted_types) = self
            .registry
            .discover_needed(
                media_type,
                storefront,
                external_id,
                title,
                DiscoverOptions {
                    existing_counts: &existing_counts,
                    searched_types: &searched_types,
                    limit,
                    force,
                },
            )
            .await?;

        let mut assets_to_process = Vec::new();
        for asset in discovered_assets {
            let asset_type_id = asset.asset_type.id();
            let count = existing_counts.entry(asset_type_id).or_default();
            if let Some(max) = limit {
                if *count < max {
                    *count += 1;
                    assets_to_process.push(asset);
                }
            } else {
                assets_to_process.push(asset);
            }
        }

        for asset in assets_to_process {
            let path = self
                .downloader
                .download(
                    Uuid::parse_str(&media.id).unwrap(),
                    asset.asset_type,
                    &asset.url,
                )
                .await?;

            insert_media_asset(
                &self.pool,
                MediaAssetInsert {
                    media_id: media.id.clone(),
                    path: path.to_string_lossy().into_owned(),
                    asset_id: asset.asset_type.id(),
                },
            )
            .await?;
        }

        for asset_type_id in attempted_types {
            insert_media_asset_search(
                &self.pool,
                MediaAssetSearchInsert {
                    media_id: media.id.clone(),
                    asset_id: asset_type_id,
                },
            )
            .await?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use tempfile::tempdir;
    use wiremock::{
        Mock, MockServer, ResponseTemplate,
        matchers::{method, path},
    };

    use itonda_database::{
        media::{MediaInsert, find_assets_by_media_ids, insert_media},
        test_utils::setup_db,
    };

    use crate::{
        assets::{
            error::AssetError,
            models::{AssetStoreId, PosterSearchOptions},
            policy::AssetPolicy,
            traits::{AssetFetcher, BannerFetcher, PosterFetcher},
            types::AssetType,
        },
        media::{discovered::DiscoveredAsset, models::Media, types::MediaStatus, types::MediaType},
        storage::path::AppPaths,
        storefronts::models::StorefrontId,
        tests::fixtures::media::DiscoveredMediaBuilder,
    };

    struct IndividualPosterFetcher {
        url: String,
        support_all: bool,
    }

    impl IndividualPosterFetcher {
        fn new(url: String) -> Self {
            Self {
                url,
                support_all: false,
            }
        }

        fn supporting_all(url: String) -> Self {
            Self {
                url,
                support_all: true,
            }
        }
    }

    impl AssetFetcher for IndividualPosterFetcher {
        fn id(&self) -> AssetStoreId {
            AssetStoreId::SteamGridDb
        }

        fn supports_media_type(&self, media_type: MediaType) -> bool {
            if self.support_all {
                true
            } else {
                matches!(media_type, MediaType::Game)
            }
        }
    }

    #[async_trait]
    impl PosterFetcher for IndividualPosterFetcher {
        async fn discover_poster(
            &self,
            _media_type: Option<MediaType>,
            _storefront: Option<StorefrontId>,
            _external_id: Option<&str>,
            _title: &str,
        ) -> Result<Option<DiscoveredAsset>, AssetError> {
            Ok(Some(DiscoveredAsset {
                asset_type: AssetType::Poster,
                url: self.url.clone(),
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
                url: self.url.clone(),
            }])
        }
    }

    #[test]
    fn policy_defaults_to_first_only() {
        assert_eq!(AssetPolicy::default(), AssetPolicy::FirstOnly);
        assert_eq!(AssetPolicy::FirstOnly.max_items(), Some(1));
    }

    #[test]
    fn policy_truncates_items_correctly() {
        let items = vec![1, 2, 3];

        assert_eq!(AssetPolicy::FirstOnly.apply(items.clone()), vec![1]);
        assert_eq!(AssetPolicy::All.apply(items.clone()), vec![1, 2, 3]);
        assert_eq!(AssetPolicy::Limit(2).apply(items.clone()), vec![1, 2]);
    }

    #[tokio::test]
    async fn step_uses_first_only_policy_by_default() {
        let pool = setup_db().await;
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/poster1.png"))
            .respond_with(ResponseTemplate::new(200).set_body_bytes(b"image1"))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/poster2.png"))
            .respond_with(ResponseTemplate::new(200).set_body_bytes(b"image2"))
            .mount(&server)
            .await;

        let temp = tempdir().unwrap();
        let paths = AppPaths {
            config_dir: temp.path().join("config"),
            data_dir: temp.path().join("data"),
        };

        let mut registry = AssetRegistry::new();
        registry.register_poster(Arc::new(IndividualPosterFetcher::new(format!(
            "{}/poster1.png",
            server.uri()
        ))));
        registry.register_poster(Arc::new(IndividualPosterFetcher::new(format!(
            "{}/poster2.png",
            server.uri()
        ))));

        let downloader = AssetDownloader::new(paths);
        let step = AssetStep::new(pool.clone(), registry, downloader);

        assert_eq!(step.policy(), AssetPolicy::FirstOnly);

        let media_row = insert_media(
            &pool,
            MediaInsert {
                title: "Test Game".into(),
                media_type: "game".into(),
                status_id: MediaStatus::NotStarted.id(),
                ..Default::default()
            },
        )
        .await
        .unwrap();
        let media = Media::try_from(media_row).unwrap();

        let discovered = DiscoveredMediaBuilder::new().title("Test Game").build();
        let mut context = SyncContext::new(discovered, false);
        context.media = Some(media.clone());

        step.execute(&mut context).await.unwrap();

        let assets = find_assets_by_media_ids(&pool, &[media.id]).await.unwrap();
        assert_eq!(assets.len(), 1);
    }

    #[tokio::test]
    async fn step_fetches_assets_for_non_api_media() {
        let pool = setup_db().await;
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/mr_robot.png"))
            .respond_with(ResponseTemplate::new(200).set_body_bytes(b"mrrobotimage"))
            .mount(&server)
            .await;

        let temp = tempdir().unwrap();
        let paths = AppPaths {
            config_dir: temp.path().join("config"),
            data_dir: temp.path().join("data"),
        };

        let mut registry = AssetRegistry::new();
        registry.register_poster(Arc::new(IndividualPosterFetcher::supporting_all(format!(
            "{}/mr_robot.png",
            server.uri()
        ))));

        let downloader = AssetDownloader::new(paths);
        let step = AssetStep::new(pool.clone(), registry, downloader);

        let media_row = insert_media(
            &pool,
            MediaInsert {
                title: "Mr. Robot".into(),
                media_type: "tv_show".into(),
                status_id: MediaStatus::NotStarted.id(),
                ..Default::default()
            },
        )
        .await
        .unwrap();
        let media = Media::try_from(media_row).unwrap();

        let mut context = SyncContext::from_media(media.clone());

        step.execute(&mut context).await.unwrap();

        let assets = find_assets_by_media_ids(&pool, &[media.id]).await.unwrap();
        assert_eq!(assets.len(), 1);
    }

    #[tokio::test]
    async fn step_respects_all_policy() {
        let pool = setup_db().await;
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/poster1.png"))
            .respond_with(ResponseTemplate::new(200).set_body_bytes(b"image1"))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/poster2.png"))
            .respond_with(ResponseTemplate::new(200).set_body_bytes(b"image2"))
            .mount(&server)
            .await;

        let temp = tempdir().unwrap();
        let paths = AppPaths {
            config_dir: temp.path().join("config"),
            data_dir: temp.path().join("data"),
        };

        let mut registry = AssetRegistry::new();
        registry.register_poster(Arc::new(IndividualPosterFetcher::new(format!(
            "{}/poster1.png",
            server.uri()
        ))));
        registry.register_poster(Arc::new(IndividualPosterFetcher::new(format!(
            "{}/poster2.png",
            server.uri()
        ))));

        let downloader = AssetDownloader::new(paths);
        let step = AssetStep::with_policy(pool.clone(), registry, downloader, AssetPolicy::All);

        let media_row = insert_media(
            &pool,
            MediaInsert {
                title: "Test Game".into(),
                media_type: "game".into(),
                status_id: MediaStatus::NotStarted.id(),
                ..Default::default()
            },
        )
        .await
        .unwrap();
        let media = Media::try_from(media_row).unwrap();

        let discovered = DiscoveredMediaBuilder::new().title("Test Game").build();
        let mut context = SyncContext::new(discovered, false);
        context.media = Some(media.clone());

        step.execute(&mut context).await.unwrap();

        let assets = find_assets_by_media_ids(&pool, &[media.id]).await.unwrap();
        assert_eq!(assets.len(), 2);
    }

    #[tokio::test]
    async fn step_skips_discovery_when_asset_limit_reached() {
        let pool = setup_db().await;

        let temp = tempdir().unwrap();
        let paths = AppPaths {
            config_dir: temp.path().join("config"),
            data_dir: temp.path().join("data"),
        };

        let registry = AssetRegistry::new();
        let downloader = AssetDownloader::new(paths);
        let step = AssetStep::new(pool.clone(), registry, downloader);

        let media_row = insert_media(
            &pool,
            MediaInsert {
                title: "Test Game".into(),
                media_type: "game".into(),
                status_id: MediaStatus::NotStarted.id(),
                ..Default::default()
            },
        )
        .await
        .unwrap();
        let media = Media::try_from(media_row).unwrap();

        insert_media_asset(
            &pool,
            MediaAssetInsert {
                media_id: media.id.clone(),
                path: "/path/to/existing_poster.png".into(),
                asset_id: AssetType::Poster.id(),
            },
        )
        .await
        .unwrap();

        let discovered = DiscoveredMediaBuilder::new().title("Test Game").build();
        let mut context = SyncContext::new(discovered, false);
        context.media = Some(media.clone());

        step.execute(&mut context).await.unwrap();

        let assets = find_assets_by_media_ids(&pool, &[media.id]).await.unwrap();
        assert_eq!(assets.len(), 1);
        assert_eq!(assets[0].path, "/path/to/existing_poster.png");
    }

    struct CustomAssetFetcher {
        asset_type: AssetType,
        url: String,
    }

    impl CustomAssetFetcher {
        fn new(asset_type: AssetType, url: String) -> Self {
            Self { asset_type, url }
        }
    }

    impl AssetFetcher for CustomAssetFetcher {
        fn id(&self) -> AssetStoreId {
            AssetStoreId::SteamGridDb
        }

        fn supports_media_type(&self, _media_type: MediaType) -> bool {
            true
        }
    }

    #[async_trait]
    impl PosterFetcher for CustomAssetFetcher {
        fn discovered_asset_types(&self) -> Vec<AssetType> {
            vec![self.asset_type]
        }

        async fn discover_poster(
            &self,
            _media_type: Option<MediaType>,
            _storefront: Option<StorefrontId>,
            _external_id: Option<&str>,
            _title: &str,
        ) -> Result<Option<DiscoveredAsset>, AssetError> {
            Ok(Some(DiscoveredAsset {
                asset_type: self.asset_type,
                url: self.url.clone(),
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
                asset_type: self.asset_type,
                url: self.url.clone(),
            }])
        }
    }

    struct CustomBannerFetcher {
        asset_type: AssetType,
        url: String,
    }

    impl CustomBannerFetcher {
        fn new(asset_type: AssetType, url: String) -> Self {
            Self { asset_type, url }
        }
    }

    impl AssetFetcher for CustomBannerFetcher {
        fn id(&self) -> AssetStoreId {
            AssetStoreId::SteamGridDb
        }

        fn supports_media_type(&self, _media_type: MediaType) -> bool {
            true
        }
    }

    #[async_trait]
    impl BannerFetcher for CustomBannerFetcher {
        fn discovered_asset_types(&self) -> Vec<AssetType> {
            vec![self.asset_type]
        }

        async fn discover_banner(
            &self,
            _media_type: Option<MediaType>,
            _storefront: Option<StorefrontId>,
            _external_id: Option<&str>,
            _title: &str,
        ) -> Result<Option<DiscoveredAsset>, AssetError> {
            Ok(Some(DiscoveredAsset {
                asset_type: self.asset_type,
                url: self.url.clone(),
            }))
        }

        async fn search_banner(
            &self,
            _media_type: Option<MediaType>,
            _storefront: Option<StorefrontId>,
            _external_id: Option<&str>,
            _title: &str,
            _options: &PosterSearchOptions,
        ) -> Result<Vec<DiscoveredAsset>, AssetError> {
            Ok(vec![DiscoveredAsset {
                asset_type: self.asset_type,
                url: self.url.clone(),
            }])
        }
    }

    #[tokio::test]
    async fn step_downloads_missing_grid_when_another_asset_type_exists() {
        let pool = setup_db().await;
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/poster1.png"))
            .respond_with(ResponseTemplate::new(200).set_body_bytes(b"poster1"))
            .mount(&server)
            .await;

        let temp = tempdir().unwrap();
        let paths = AppPaths {
            config_dir: temp.path().join("config"),
            data_dir: temp.path().join("data"),
        };

        let mut registry = AssetRegistry::new();
        registry.register_poster(Arc::new(CustomAssetFetcher::new(
            AssetType::Poster,
            format!("{}/poster1.png", server.uri()),
        )));

        let downloader = AssetDownloader::new(paths);
        let step = AssetStep::new(pool.clone(), registry, downloader);

        let media_row = insert_media(
            &pool,
            MediaInsert {
                title: "Test Game".into(),
                media_type: "game".into(),
                status_id: MediaStatus::NotStarted.id(),
                ..Default::default()
            },
        )
        .await
        .unwrap();
        let media = Media::try_from(media_row).unwrap();

        insert_media_asset(
            &pool,
            MediaAssetInsert {
                media_id: media.id.clone(),
                path: "/path/to/existing_icon.png".into(),
                asset_id: AssetType::Icon.id(),
            },
        )
        .await
        .unwrap();

        let mut context = SyncContext::from_media(media.clone());
        step.execute(&mut context).await.unwrap();

        let assets = find_assets_by_media_ids(&pool, &[media.id]).await.unwrap();
        assert_eq!(assets.len(), 2);
        let asset_ids: Vec<i64> = assets.iter().map(|a| a.asset_id).collect();
        assert!(asset_ids.contains(&AssetType::Icon.id()));
        assert!(asset_ids.contains(&AssetType::Poster.id()));
    }

    #[tokio::test]
    async fn step_downloads_missing_banner_when_poster_exists() {
        let pool = setup_db().await;
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/banner1.png"))
            .respond_with(ResponseTemplate::new(200).set_body_bytes(b"banner1"))
            .mount(&server)
            .await;

        let temp = tempdir().unwrap();
        let paths = AppPaths {
            config_dir: temp.path().join("config"),
            data_dir: temp.path().join("data"),
        };

        let mut registry = AssetRegistry::new();
        registry.register_banner(Arc::new(CustomBannerFetcher::new(
            AssetType::Banner,
            format!("{}/banner1.png", server.uri()),
        )));

        let downloader = AssetDownloader::new(paths);
        let step = AssetStep::new(pool.clone(), registry, downloader);

        let media_row = insert_media(
            &pool,
            MediaInsert {
                title: "Test Game".into(),
                media_type: "game".into(),
                status_id: MediaStatus::NotStarted.id(),
                ..Default::default()
            },
        )
        .await
        .unwrap();
        let media = Media::try_from(media_row).unwrap();

        insert_media_asset(
            &pool,
            MediaAssetInsert {
                media_id: media.id.clone(),
                path: "/path/to/existing_poster.png".into(),
                asset_id: AssetType::Poster.id(),
            },
        )
        .await
        .unwrap();

        let mut context = SyncContext::from_media(media.clone());
        step.execute(&mut context).await.unwrap();

        let assets = find_assets_by_media_ids(&pool, &[media.id]).await.unwrap();
        assert_eq!(assets.len(), 2);
        let asset_ids: Vec<i64> = assets.iter().map(|a| a.asset_id).collect();
        assert!(asset_ids.contains(&AssetType::Poster.id()));
        assert!(asset_ids.contains(&AssetType::Banner.id()));
    }

    #[tokio::test]
    async fn step_limits_assets_per_asset_type() {
        let pool = setup_db().await;
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/poster1.png"))
            .respond_with(ResponseTemplate::new(200).set_body_bytes(b"poster1"))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/poster2.png"))
            .respond_with(ResponseTemplate::new(200).set_body_bytes(b"poster2"))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/backdrop1.png"))
            .respond_with(ResponseTemplate::new(200).set_body_bytes(b"backdrop1"))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/backdrop2.png"))
            .respond_with(ResponseTemplate::new(200).set_body_bytes(b"backdrop2"))
            .mount(&server)
            .await;

        let temp = tempdir().unwrap();
        let paths = AppPaths {
            config_dir: temp.path().join("config"),
            data_dir: temp.path().join("data"),
        };

        let mut registry = AssetRegistry::new();
        registry.register_poster(Arc::new(CustomAssetFetcher::new(
            AssetType::Poster,
            format!("{}/poster1.png", server.uri()),
        )));
        registry.register_poster(Arc::new(CustomAssetFetcher::new(
            AssetType::Poster,
            format!("{}/poster2.png", server.uri()),
        )));
        registry.register_poster(Arc::new(CustomAssetFetcher::new(
            AssetType::Backdrop,
            format!("{}/backdrop1.png", server.uri()),
        )));
        registry.register_poster(Arc::new(CustomAssetFetcher::new(
            AssetType::Backdrop,
            format!("{}/backdrop2.png", server.uri()),
        )));

        let downloader = AssetDownloader::new(paths);
        let step = AssetStep::new(pool.clone(), registry, downloader);

        let media_row = insert_media(
            &pool,
            MediaInsert {
                title: "Test Game".into(),
                media_type: "game".into(),
                status_id: MediaStatus::NotStarted.id(),
                ..Default::default()
            },
        )
        .await
        .unwrap();
        let media = Media::try_from(media_row).unwrap();

        let mut context = SyncContext::from_media(media.clone());
        step.execute(&mut context).await.unwrap();

        let assets = find_assets_by_media_ids(&pool, &[media.id]).await.unwrap();
        assert_eq!(assets.len(), 2);
        let asset_ids: Vec<i64> = assets.iter().map(|a| a.asset_id).collect();
        assert!(asset_ids.contains(&AssetType::Poster.id()));
        assert!(asset_ids.contains(&AssetType::Backdrop.id()));
    }

    #[tokio::test]
    async fn step_limits_per_asset_type_considering_existing_assets() {
        let pool = setup_db().await;
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/poster2.png"))
            .respond_with(ResponseTemplate::new(200).set_body_bytes(b"poster2"))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/backdrop1.png"))
            .respond_with(ResponseTemplate::new(200).set_body_bytes(b"backdrop1"))
            .mount(&server)
            .await;

        let temp = tempdir().unwrap();
        let paths = AppPaths {
            config_dir: temp.path().join("config"),
            data_dir: temp.path().join("data"),
        };

        let mut registry = AssetRegistry::new();
        registry.register_poster(Arc::new(CustomAssetFetcher::new(
            AssetType::Poster,
            format!("{}/poster2.png", server.uri()),
        )));
        registry.register_poster(Arc::new(CustomAssetFetcher::new(
            AssetType::Backdrop,
            format!("{}/backdrop1.png", server.uri()),
        )));

        let downloader = AssetDownloader::new(paths);
        let step = AssetStep::new(pool.clone(), registry, downloader);

        let media_row = insert_media(
            &pool,
            MediaInsert {
                title: "Test Game".into(),
                media_type: "game".into(),
                status_id: MediaStatus::NotStarted.id(),
                ..Default::default()
            },
        )
        .await
        .unwrap();
        let media = Media::try_from(media_row).unwrap();

        insert_media_asset(
            &pool,
            MediaAssetInsert {
                media_id: media.id.clone(),
                path: "/path/to/existing_poster.png".into(),
                asset_id: AssetType::Poster.id(),
            },
        )
        .await
        .unwrap();

        let mut context = SyncContext::from_media(media.clone());
        step.execute(&mut context).await.unwrap();

        let assets = find_assets_by_media_ids(&pool, &[media.id]).await.unwrap();
        assert_eq!(assets.len(), 2);
        let poster_count = assets
            .iter()
            .filter(|a| a.asset_id == AssetType::Poster.id())
            .count();
        let backdrop_count = assets
            .iter()
            .filter(|a| a.asset_id == AssetType::Backdrop.id())
            .count();
        assert_eq!(poster_count, 1);
        assert_eq!(backdrop_count, 1);
    }

    struct PanicPosterFetcher;

    impl AssetFetcher for PanicPosterFetcher {
        fn id(&self) -> AssetStoreId {
            AssetStoreId::SteamGridDb
        }

        fn supports_media_type(&self, _media_type: MediaType) -> bool {
            true
        }
    }

    #[async_trait]
    impl PosterFetcher for PanicPosterFetcher {
        async fn discover_poster(
            &self,
            _media_type: Option<MediaType>,
            _storefront: Option<StorefrontId>,
            _external_id: Option<&str>,
            _title: &str,
        ) -> Result<Option<DiscoveredAsset>, AssetError> {
            panic!("discover_poster should not be called when limit is already reached!");
        }

        async fn search_poster(
            &self,
            _media_type: Option<MediaType>,
            _storefront: Option<StorefrontId>,
            _external_id: Option<&str>,
            _title: &str,
            _options: &PosterSearchOptions,
        ) -> Result<Vec<DiscoveredAsset>, AssetError> {
            panic!("search_poster should not be called when limit is already reached!");
        }
    }

    #[tokio::test]
    async fn step_skips_calling_discover_when_enough_assets_exist() {
        let pool = setup_db().await;

        let temp = tempdir().unwrap();
        let paths = AppPaths {
            config_dir: temp.path().join("config"),
            data_dir: temp.path().join("data"),
        };

        let mut registry = AssetRegistry::new();
        registry.register_poster(Arc::new(PanicPosterFetcher));

        let downloader = AssetDownloader::new(paths);
        let step = AssetStep::new(pool.clone(), registry, downloader);

        let media_row = insert_media(
            &pool,
            MediaInsert {
                title: "Test Game".into(),
                media_type: "game".into(),
                status_id: MediaStatus::NotStarted.id(),
                ..Default::default()
            },
        )
        .await
        .unwrap();
        let media = Media::try_from(media_row).unwrap();

        insert_media_asset(
            &pool,
            MediaAssetInsert {
                media_id: media.id.clone(),
                path: "/path/to/existing_poster.png".into(),
                asset_id: AssetType::Poster.id(),
            },
        )
        .await
        .unwrap();

        let mut context = SyncContext::from_media(media.clone());
        step.execute(&mut context).await.unwrap();
    }

    #[tokio::test]
    async fn step_skips_poster_discovery_when_poster_limit_reached_but_banner_missing() {
        let pool = setup_db().await;
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/banner1.png"))
            .respond_with(ResponseTemplate::new(200).set_body_bytes(b"banner1"))
            .mount(&server)
            .await;

        let temp = tempdir().unwrap();
        let paths = AppPaths {
            config_dir: temp.path().join("config"),
            data_dir: temp.path().join("data"),
        };

        let mut registry = AssetRegistry::new();
        registry.register_poster(Arc::new(PanicPosterFetcher));
        registry.register_banner(Arc::new(CustomBannerFetcher::new(
            AssetType::Banner,
            format!("{}/banner1.png", server.uri()),
        )));

        let downloader = AssetDownloader::new(paths);
        let step = AssetStep::new(pool.clone(), registry, downloader);

        let media_row = insert_media(
            &pool,
            MediaInsert {
                title: "Test Game".into(),
                media_type: "game".into(),
                status_id: MediaStatus::NotStarted.id(),
                ..Default::default()
            },
        )
        .await
        .unwrap();
        let media = Media::try_from(media_row).unwrap();

        insert_media_asset(
            &pool,
            MediaAssetInsert {
                media_id: media.id.clone(),
                path: "/path/to/existing_poster.png".into(),
                asset_id: AssetType::Poster.id(),
            },
        )
        .await
        .unwrap();

        let mut context = SyncContext::from_media(media.clone());
        step.execute(&mut context).await.unwrap();

        let assets = find_assets_by_media_ids(&pool, &[media.id]).await.unwrap();
        assert_eq!(assets.len(), 2);
        let asset_ids: Vec<i64> = assets.iter().map(|a| a.asset_id).collect();
        assert!(asset_ids.contains(&AssetType::Poster.id()));
        assert!(asset_ids.contains(&AssetType::Banner.id()));
    }
}
