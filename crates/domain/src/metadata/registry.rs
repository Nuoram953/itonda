use std::sync::Arc;

use crate::{
    media::types::MediaType,
    metadata::{
        error::MetadataError,
        models::{GeneralMetadata, MetadataProviderId, MetadataQuery},
        traits::GeneralInfoFetcher,
    },
};

#[derive(Clone, Default)]
pub struct MetadataRegistry {
    fetchers: Vec<Arc<dyn GeneralInfoFetcher>>,
}

impl MetadataRegistry {
    pub fn new() -> Self {
        Self {
            fetchers: Vec::new(),
        }
    }

    pub fn register(&mut self, fetcher: Arc<dyn GeneralInfoFetcher>) {
        self.fetchers.push(fetcher);
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
    ) -> Result<Option<GeneralMetadata>, MetadataError> {
        let mut accumulated: Option<GeneralMetadata> = None;

        for fetcher in self.fetchers_for_type(query.media_type) {
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

        Ok(accumulated)
    }

    pub async fn fetch_general_info(
        &self,
        query: &MetadataQuery<'_>,
    ) -> Result<Option<GeneralMetadata>, MetadataError> {
        self.fetch_general_info_with_policy(
            query,
            crate::metadata::policy::MetadataPolicy::default(),
        )
        .await
    }
}
