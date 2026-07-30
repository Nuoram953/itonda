use async_trait::async_trait;
use itonda_database::{
    media::{
        MediaInsert, MediaLaunchUpsert, find_media_by_title, insert_media, upsert_media_launch,
    },
    models::UpsertAction,
};
use sqlx::SqlitePool;

use crate::{
    media::models::{Media, MediaStatus, MediaType},
    sync::{
        context::{SyncAction, SyncContext},
        errors::SyncError,
        pipeline::SyncStep,
    },
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
                context.action = SyncAction::Created;
                insert_media(
                    &self.pool,
                    MediaInsert {
                        title: context.discovered.title.clone(),
                        media_type: context.discovered.media_type.as_str().into(),
                        status_id: MediaStatus::NotStarted.id(),
                    },
                )
                .await?
            }
        };

        let media = Media::try_from(row).unwrap();
        if let MediaType::Game = context.discovered.media_type
            && let Some(launch) = &context.discovered.launch
        {
            let result = upsert_media_launch(
                &self.pool,
                MediaLaunchUpsert {
                    media_id: media.id.clone(),
                    name: launch.name.clone(),
                    launch_type: launch.launch_type.as_str().into(),
                    program: launch.program.clone(),
                    arguments: serde_json::to_string(&launch.arguments)?,
                    working_directory: launch.working_directory.clone(),
                    is_default: false,
                    enabled: true,
                },
            )
            .await?;

            if result.action != UpsertAction::Unchanged {
                context.action = SyncAction::Updated;
            }
        }
        context.media = Some(media);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use itonda_database::{media::find_media_launch_by_media_id, test_utils::setup_db};

    use crate::tests::fixtures::{
        context::sync_context_with_media,
        media::{DiscoveredLaunchBuilder, DiscoveredMediaBuilder},
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
    async fn creates_storefront_relationship_for_game() {
        let pool = setup_db().await;

        let step = PersistStep::new(pool.clone());

        let media = DiscoveredMediaBuilder::new()
            .launch(
                DiscoveredLaunchBuilder::new()
                    .name("Test storefront")
                    .build(),
            )
            .build();

        let mut context = sync_context_with_media(media);

        step.execute(&mut context).await.unwrap();

        let media = context.media.unwrap();

        let storefront = find_media_launch_by_media_id(&pool, media.id)
            .await
            .unwrap();

        assert_eq!(storefront[0].name, "Test storefront");
    }
}
