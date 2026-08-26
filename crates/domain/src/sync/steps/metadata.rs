use async_trait::async_trait;
use itonda_database::media::{
    MediaGameDetailsUpsert, MediaMetadataSearchInsert, MediaMetadataUpdate, find_game_details,
    find_metadata_search_by_media_id, insert_media_metadata_search, sync_media_companies,
    sync_media_genres, sync_media_tags, update_media_metadata, upsert_media_game_details,
};
use sqlx::SqlitePool;

use crate::{
    metadata::{models::MetadataQuery, policy::MetadataPolicy, registry::MetadataRegistry},
    sync::{context::SyncContext, errors::SyncError, pipeline::SyncStep},
};

pub struct MetadataStep {
    pool: SqlitePool,
    registry: MetadataRegistry,
    policy: MetadataPolicy,
}

impl MetadataStep {
    pub fn new(pool: SqlitePool, registry: MetadataRegistry) -> Self {
        Self::with_policy(pool, registry, MetadataPolicy::default())
    }

    pub fn with_policy(
        pool: SqlitePool,
        registry: MetadataRegistry,
        policy: MetadataPolicy,
    ) -> Self {
        Self {
            pool,
            registry,
            policy,
        }
    }

    pub fn policy(&self) -> MetadataPolicy {
        self.policy
    }
}

#[async_trait]
impl SyncStep for MetadataStep {
    fn name(&self) -> &'static str {
        "Metadata"
    }

    async fn execute(&self, context: &mut SyncContext) -> Result<(), SyncError> {
        let Some(media) = &mut context.media else {
            return Ok(());
        };

        if !context.force {
            let already_searched = find_metadata_search_by_media_id(&self.pool, &media.id)
                .await?
                .is_some();

            if already_searched {
                tracing::debug!(
                    "Skipping metadata step for '{}' (already searched)",
                    media.title
                );
                return Ok(());
            }
        }

        let storefront = context.discovered.as_ref().map(|d| d.storefront);
        let external_id = context.discovered.as_ref().map(|d| d.external_id.as_str());

        let query = MetadataQuery {
            title: &media.title,
            media_type: media.media_type,
            storefront,
            external_id,
            force: context.force,
        };

        let metadata = match self
            .registry
            .fetch_general_info_with_policy(&query, self.policy)
            .await
        {
            Ok(Some(meta)) => meta,
            Ok(None) => {
                insert_media_metadata_search(
                    &self.pool,
                    MediaMetadataSearchInsert {
                        media_id: media.id.clone(),
                    },
                )
                .await?;
                return Ok(());
            }
            Err(err) => {
                tracing::warn!("Metadata fetch failed for {}: {err}", media.title);
                return Ok(());
            }
        };

        let common = metadata.common();

        update_media_metadata(
            &self.pool,
            MediaMetadataUpdate {
                media_id: media.id.clone(),
                description: common.description.clone(),
                summary: common.summary.clone(),
                release_date: common.release_date,
            },
        )
        .await?;

        if common.description.is_some() {
            media.description = common.description.clone();
        }
        if common.summary.is_some() {
            media.summary = common.summary.clone();
        }
        if common.release_date.is_some() {
            media.release_date = common.release_date;
        }

        if !common.genres.is_empty() {
            sync_media_genres(&self.pool, &media.id, &common.genres).await?;
            for g in &common.genres {
                if !media.genres.contains(g) {
                    media.genres.push(g.clone());
                }
            }
        }

        if !common.tags.is_empty() {
            sync_media_tags(&self.pool, &media.id, &common.tags).await?;
            for t in &common.tags {
                if !media.tags.contains(t) {
                    media.tags.push(t.clone());
                }
            }
        }

        match metadata {
            crate::metadata::models::GeneralMetadata::Game(game_meta) => {
                if !game_meta.developers.is_empty() || !game_meta.publishers.is_empty() {
                    sync_media_companies(
                        &self.pool,
                        &media.id,
                        &game_meta.developers,
                        &game_meta.publishers,
                    )
                    .await?;
                }

                if game_meta.series.is_some() {
                    let existing = find_game_details(&self.pool, &media.id).await?;
                    let playtime = existing.as_ref().and_then(|e| e.playtime_minutes);
                    let last_played = existing.as_ref().and_then(|e| e.last_played_at);

                    upsert_media_game_details(
                        &self.pool,
                        MediaGameDetailsUpsert {
                            media_id: media.id.clone(),
                            playtime_minutes: playtime,
                            last_played_at: last_played,
                            series: game_meta.series.clone(),
                        },
                    )
                    .await?;

                    if let Some(crate::media::models::MediaDetails::Game(details)) =
                        &mut media.details
                    {
                        details.series = game_meta.series.clone();
                        for dev in &game_meta.developers {
                            if !details.developers.contains(dev) {
                                details.developers.push(dev.clone());
                            }
                        }
                        for publ in &game_meta.publishers {
                            if !details.publishers.contains(publ) {
                                details.publishers.push(publ.clone());
                            }
                        }
                    }
                }
            }
        }

        insert_media_metadata_search(
            &self.pool,
            MediaMetadataSearchInsert {
                media_id: media.id.clone(),
            },
        )
        .await?;

        Ok(())
    }
}
