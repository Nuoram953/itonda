use std::sync::Arc;

use crate::{
    media::types::MediaType,
    metadata::{
        error::MetadataError,
        models::{GeneralMetadata, MediaSearchResult, MetadataProviderId, MetadataQuery},
        traits::{GeneralInfoFetcher, MediaSearcher},
    },
};

#[derive(Clone, Default)]
pub struct MetadataRegistry {
    fetchers: Vec<Arc<dyn GeneralInfoFetcher>>,
    searchers: Vec<Arc<dyn MediaSearcher>>,
}

impl MetadataRegistry {
    pub fn new() -> Self {
        Self {
            fetchers: Vec::new(),
            searchers: Vec::new(),
        }
    }

    pub fn register(&mut self, fetcher: Arc<dyn GeneralInfoFetcher>) {
        self.fetchers.push(fetcher);
    }

    pub fn register_searcher(&mut self, searcher: Arc<dyn MediaSearcher>) {
        self.searchers.push(searcher);
    }

    pub fn get(&self, id: MetadataProviderId) -> Option<Arc<dyn GeneralInfoFetcher>> {
        self.fetchers.iter().find(|f| f.id() == id).cloned()
    }

    pub fn fetchers_for_type(&self, media_type: MediaType) -> Vec<Arc<dyn GeneralInfoFetcher>> {
        self.fetchers
            .iter()
            .filter(|f| f.supports_media_type(media_type))
            .cloned()
            .collect()
    }

    pub async fn fetch_general_info_with_policy(
        &self,
        query: &MetadataQuery<'_>,
        policy: crate::metadata::policy::MetadataPolicy,
        searched_stores: &std::collections::HashSet<String>,
    ) -> Result<(Option<GeneralMetadata>, Vec<MetadataProviderId>), MetadataError> {
        let mut accumulated: Option<GeneralMetadata> = None;
        let mut attempted: Vec<MetadataProviderId> = Vec::new();

        for fetcher in self.fetchers_for_type(query.media_type) {
            let store_id = fetcher.id().as_str();
            if !query.force && searched_stores.contains(store_id) {
                continue;
            }

            attempted.push(fetcher.id());

            match fetcher.fetch_general_info(query).await {
                Ok(Some(meta)) => {
                    if let Some(acc) = &mut accumulated {
                        acc.merge(meta);
                    } else {
                        accumulated = Some(meta);
                    }

                    if let Some(acc) = &accumulated
                        && policy.is_satisfied(acc)
                    {
                        break;
                    }
                }
                Ok(None) => continue,
                Err(err) => {
                    tracing::warn!("Metadata fetcher {} error: {err}", fetcher.name());
                    continue;
                }
            }
        }

        Ok((accumulated, attempted))
    }

    pub async fn fetch_general_info(
        &self,
        query: &MetadataQuery<'_>,
    ) -> Result<Option<GeneralMetadata>, MetadataError> {
        let (meta, _) = self
            .fetch_general_info_with_policy(
                query,
                crate::metadata::policy::MetadataPolicy::default(),
                &std::collections::HashSet::new(),
            )
            .await?;
        Ok(meta)
    }

    pub async fn search(
        &self,
        query: &str,
        media_type: MediaType,
    ) -> Result<Vec<MediaSearchResult>, MetadataError> {
        for searcher in &self.searchers {
            if searcher.supports_media_type(media_type) {
                return searcher.search(query, media_type).await;
            }
        }
        Ok(Vec::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;

    struct MockSearcher {
        supported: MediaType,
        results: Vec<MediaSearchResult>,
    }

    #[async_trait]
    impl MediaSearcher for MockSearcher {
        fn supports_media_type(&self, media_type: MediaType) -> bool {
            self.supported == media_type
        }

        async fn search(
            &self,
            _query: &str,
            _media_type: MediaType,
        ) -> Result<Vec<MediaSearchResult>, MetadataError> {
            Ok(self.results.clone())
        }
    }

    #[tokio::test]
    async fn test_registry_search_dispatches_by_media_type() {
        let mut registry = MetadataRegistry::new();
        registry.register_searcher(Arc::new(MockSearcher {
            supported: MediaType::Game,
            results: vec![MediaSearchResult {
                external_id: "1".into(),
                title: "Mock Game".into(),
                media_type: MediaType::Game,
                year: Some(2023),
                summary: None,
                cover_url: None,
            }],
        }));
        registry.register_searcher(Arc::new(MockSearcher {
            supported: MediaType::Movie,
            results: vec![MediaSearchResult {
                external_id: "2".into(),
                title: "Mock Movie".into(),
                media_type: MediaType::Movie,
                year: Some(2024),
                summary: None,
                cover_url: None,
            }],
        }));

        let game_results = registry.search("test", MediaType::Game).await.unwrap();
        assert_eq!(game_results.len(), 1);
        assert_eq!(game_results[0].title, "Mock Game");

        let movie_results = registry.search("test", MediaType::Movie).await.unwrap();
        assert_eq!(movie_results.len(), 1);
        assert_eq!(movie_results[0].title, "Mock Movie");

        let tv_results = registry.search("test", MediaType::TvShow).await.unwrap();
        assert_eq!(tv_results.len(), 0);
    }
}
