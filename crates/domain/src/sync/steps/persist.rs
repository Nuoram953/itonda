use async_trait::async_trait;
use itonda_database::media::{
    MediaGameStorefrontUpsert, MediaInsert, find_media_by_title, insert_media,
    upsert_media_game_storefront,
};
use sqlx::SqlitePool;

use crate::{
    media::models::{Media, MediaType},
    sync::{context::SyncContext, errors::SyncError, pipeline::SyncStep},
};

pub struct PersistStep {
    pool: SqlitePool,
}

impl PersistStep {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl SyncStep for PersistStep {
    fn name(&self) -> &'static str {
        "Persist"
    }

    async fn execute(&self, context: &mut SyncContext) -> Result<(), SyncError> {
        let row = match find_media_by_title(&self.pool, context.discovered.title.clone()).await? {
            Some(row) => row,
            None => {
                insert_media(
                    &self.pool,
                    MediaInsert {
                        title: context.discovered.title.clone(),
                        media_type: context.discovered.media_type.as_str().into(),
                    },
                )
                .await?
            }
        };

        let media = Media::try_from(row).unwrap();

        if let MediaType::Game = context.discovered.media_type {
            upsert_media_game_storefront(
                &self.pool,
                MediaGameStorefrontUpsert {
                    media_id: media.id.clone(),
                    storefront_id: context.discovered.storefront.into(),
                    internal_id: context.discovered.external_id.clone(),
                },
            )
            .await?;
        }

        context.media = Some(media);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use itonda_database::{media::find_media_game_storefront, test_utils::setup_db};

    use crate::{
        media::models::{DiscoveredMedia, DiscoveredMediaMetadata},
        storefronts::models::StorefrontId,
    };

    use super::*;

    #[tokio::test]
    async fn creates_media_when_missing() {
        let pool = setup_db().await;

        let step = PersistStep::new(pool.clone());

        let mut context = SyncContext {
            discovered: DiscoveredMedia {
                title: "Portal 2".into(),
                media_type: MediaType::Game,
                storefront: StorefrontId::Steam,
                external_id: "620".into(),
                metadata: DiscoveredMediaMetadata {
                    total_playtime: None,
                },
            },
            media: None,
        };

        step.execute(&mut context).await.unwrap();

        assert!(context.media.is_some());

        let media = context.media.unwrap();

        assert_eq!(media.title, "Portal 2");
    }

    #[tokio::test]
    async fn uses_existing_media() {
        let pool = setup_db().await;

        insert_media(
            &pool,
            MediaInsert {
                title: "Portal 2".into(),
                media_type: "game".into(),
            },
        )
        .await
        .unwrap();

        let step = PersistStep::new(pool.clone());

        let mut context = SyncContext {
            discovered: DiscoveredMedia {
                title: "Portal 2".into(),
                media_type: MediaType::Game,
                storefront: StorefrontId::Steam,
                external_id: "620".into(),
                metadata: DiscoveredMediaMetadata {
                    total_playtime: None,
                },
            },
            media: None,
        };

        step.execute(&mut context).await.unwrap();

        assert!(context.media.is_some());
        assert_eq!(context.media.unwrap().title, "Portal 2");
    }

    #[tokio::test]
    async fn creates_storefront_relationship_for_game() {
        let pool = setup_db().await;

        let step = PersistStep::new(pool.clone());

        let mut context = SyncContext {
            discovered: DiscoveredMedia {
                title: "Portal 2".into(),
                media_type: MediaType::Game,
                storefront: StorefrontId::Steam,
                external_id: "620".into(),
                metadata: DiscoveredMediaMetadata {
                    total_playtime: None,
                },
            },
            media: None,
        };

        step.execute(&mut context).await.unwrap();

        let media = context.media.unwrap();

        let storefront = find_media_game_storefront(&pool, media.id, StorefrontId::Steam.into())
            .await
            .unwrap();

        assert_eq!(storefront.unwrap().internal_id, "620");
    }
}
