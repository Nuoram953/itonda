use async_trait::async_trait;
use itonda_database::media::{
    MediaGameDetailsUpsert, MediaInsert, find_media_by_title, insert_media,
    upsert_media_game_details,
};
use sqlx::SqlitePool;

use crate::{
    media::{
        discovered::{DiscoveredMedia, DiscoveredMediaMetadata},
        models::Media,
        types::{MediaStatus, MediaType},
    },
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
        let Some(discovered) = &context.discovered else {
            return Ok(());
        };

        let DiscoveredMedia {
            title,
            media_type,
            metadata,
            ..
        } = discovered;

        let media = match &context.media {
            Some(media) => media.clone(),
            None => {
                let existing = find_media_by_title(&self.pool, title.clone()).await?;

                let row = match existing {
                    Some(row) => row,
                    None => {
                        insert_media(
                            &self.pool,
                            MediaInsert {
                                title: title.clone(),
                                media_type: media_type.as_str().into(),
                                status_id: MediaStatus::NotStarted.id(),
                            },
                        )
                        .await?
                    }
                };

                let media = Media::try_from(row).unwrap();
                context.media = Some(media.clone());
                media
            }
        };

        if let MediaType::Game = media_type {
            let DiscoveredMediaMetadata::Game(game) = metadata;

            let result = upsert_media_game_details(
                &self.pool,
                MediaGameDetailsUpsert {
                    media_id: media.id.clone(),
                    playtime_minutes: game.total_playtime.map(|v| v as i64),
                    last_played_at: game.last_played,
                },
            )
            .await?;

            context.action.merge(result.action.into());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use itonda_database::{media::find_game_details, test_utils::setup_db};

    use crate::{
        media::discovered::{DiscoveredMediaMetadata, GameMetadata},
        tests::fixtures::{context::sync_context_with_media, media::DiscoveredMediaBuilder},
    };

    use super::*;

    #[tokio::test]
    async fn creates_media_when_missing() {
        let pool = setup_db().await;

        let step = PersistStep::new(pool.clone());

        let media = DiscoveredMediaBuilder::new().title("Test 1").build();

        let mut context = sync_context_with_media(media);

        step.execute(&mut context).await.unwrap();

        assert!(context.media.is_some());

        let media = context.media.unwrap();

        assert_eq!(media.title, "Test 1");
    }

    #[tokio::test]
    async fn uses_existing_media() {
        let pool = setup_db().await;

        insert_media(
            &pool,
            MediaInsert {
                title: "Portal 2".into(),
                media_type: "game".into(),
                status_id: MediaStatus::NotStarted.id(),
            },
        )
        .await
        .unwrap();

        let step = PersistStep::new(pool.clone());

        let media = DiscoveredMediaBuilder::new().title("Test 1").build();

        let mut context = sync_context_with_media(media);

        step.execute(&mut context).await.unwrap();

        assert!(context.media.is_some());
        assert_eq!(context.media.unwrap().title, "Test 1");
    }

    #[tokio::test]
    async fn creates_game_details_for_game() {
        let pool = setup_db().await;

        let step = PersistStep::new(pool.clone());

        let media = DiscoveredMediaBuilder::new()
            .metadata(DiscoveredMediaMetadata::Game(GameMetadata {
                total_playtime: Some(120),
                last_played: None,
            }))
            .build();

        let mut context = sync_context_with_media(media);

        step.execute(&mut context).await.unwrap();

        let media = context.media.unwrap();

        let details = find_game_details(&pool, &media.id).await.unwrap().unwrap();

        assert_eq!(details.playtime_minutes, Some(120));
    }
}
