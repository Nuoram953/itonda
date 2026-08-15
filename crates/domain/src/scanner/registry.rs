use std::sync::Arc;
use tracing::{debug, warn};

use crate::{
    media::types::MediaType,
    scanner::{models::ScannedMedia, traits::MediaScanner},
};

#[derive(Clone, Default)]
pub struct ScannerRegistry {
    scanners: Vec<Arc<dyn MediaScanner>>,
}

impl ScannerRegistry {
    pub fn new() -> Self {
        Self {
            scanners: Vec::new(),
        }
    }

    pub fn register(&mut self, scanner: Arc<dyn MediaScanner>) {
        self.scanners.push(scanner);
    }

    pub fn scanners(&self) -> &[Arc<dyn MediaScanner>] {
        &self.scanners
    }

    pub async fn scan_all(&self) -> Vec<ScannedMedia> {
        let mut results = Vec::new();

        for scanner in &self.scanners {
            if !scanner.is_available() {
                debug!(
                    "Scanner '{}' is not available on this host. Skipping.",
                    scanner.name()
                );
                continue;
            }

            match scanner.scan().await {
                Ok(items) => {
                    debug!("Scanner '{}' found {} items", scanner.name(), items.len());
                    results.extend(items);
                }
                Err(err) => {
                    warn!("Scanner '{}' failed: {err}", scanner.name());
                }
            }
        }

        results
    }

    pub async fn scan_media_type(&self, media_type: MediaType) -> Vec<ScannedMedia> {
        let mut results = Vec::new();

        for scanner in &self.scanners {
            if scanner.is_available() && scanner.supported_media_type().contains(&media_type) {
                match scanner.scan().await {
                    Ok(items) => {
                        results.extend(items);
                    }
                    Err(err) => {
                        warn!("Scanner '{}' failed: {err}", scanner.name());
                    }
                }
            }
        }

        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::media::types::{MediaLaunchType, MediaType};
    use crate::scanner::errors::ScannerError;
    use crate::scanner::models::ScannedLaunch;
    use async_trait::async_trait;

    struct MockGameScanner;

    #[async_trait]
    impl MediaScanner for MockGameScanner {
        fn supported_media_type(&self) -> Vec<MediaType> {
            vec![MediaType::Game]
        }
        fn name(&self) -> &'static str {
            "Mock Game Scanner"
        }
        fn is_available(&self) -> bool {
            true
        }
        async fn scan(&self) -> Result<Vec<ScannedMedia>, ScannerError> {
            Ok(vec![ScannedMedia {
                media_type: MediaType::Game,
                title: "Portal 2".into(),
                external_id: Some("620".into()),
                source: "steam".into(),
                working_directory: Some("/games/portal2".into()),
                launch: Some(ScannedLaunch {
                    name: "Steam".into(),
                    launch_type: MediaLaunchType::Storefront,
                    program: "steam".into(),
                    arguments: vec!["steam://run/620".into()],
                    working_directory: Some("/games/portal2".into()),
                }),
            }])
        }
    }

    struct MockMovieScanner;

    #[async_trait]
    impl MediaScanner for MockMovieScanner {
        fn supported_media_type(&self) -> Vec<MediaType> {
            vec![MediaType::Movie]
        }
        fn name(&self) -> &'static str {
            "Mock Movie Scanner"
        }
        fn is_available(&self) -> bool {
            true
        }
        async fn scan(&self) -> Result<Vec<ScannedMedia>, ScannerError> {
            Ok(vec![ScannedMedia {
                media_type: MediaType::Movie,
                title: "Inception".into(),
                external_id: None,
                source: "local".into(),
                working_directory: Some("/movies/Inception (2010)".into()),
                launch: Some(ScannedLaunch {
                    name: "Default Player".into(),
                    launch_type: MediaLaunchType::Custom,
                    program: "mpv".into(),
                    arguments: vec!["/movies/Inception (2010)/Inception.mkv".into()],
                    working_directory: Some("/movies/Inception (2010)".into()),
                }),
            }])
        }
    }

    struct UnavailableScanner;

    #[async_trait]
    impl MediaScanner for UnavailableScanner {
        fn supported_media_type(&self) -> Vec<MediaType> {
            vec![MediaType::Game]
        }
        fn name(&self) -> &'static str {
            "Unavailable Scanner"
        }
        fn is_available(&self) -> bool {
            false
        }
        async fn scan(&self) -> Result<Vec<ScannedMedia>, ScannerError> {
            panic!("Should not be called because is_available is false");
        }
    }

    #[tokio::test]
    async fn test_scan_all_aggregates_available_scanners() {
        let mut registry = ScannerRegistry::new();
        registry.register(Arc::new(MockGameScanner));
        registry.register(Arc::new(MockMovieScanner));
        registry.register(Arc::new(UnavailableScanner));

        let items = registry.scan_all().await;
        assert_eq!(items.len(), 2);
        assert_eq!(items[0].title, "Portal 2");
        assert_eq!(items[1].title, "Inception");
    }

    #[tokio::test]
    async fn test_scan_media_type_filters_properly() {
        let mut registry = ScannerRegistry::new();
        registry.register(Arc::new(MockGameScanner));
        registry.register(Arc::new(MockMovieScanner));

        let game_items = registry.scan_media_type(MediaType::Game).await;
        assert_eq!(game_items.len(), 1);
        assert_eq!(game_items[0].title, "Portal 2");

        let movie_items = registry.scan_media_type(MediaType::Movie).await;
        assert_eq!(movie_items.len(), 1);
        assert_eq!(movie_items[0].title, "Inception");
    }
}
