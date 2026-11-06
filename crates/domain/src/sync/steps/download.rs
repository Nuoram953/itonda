use async_trait::async_trait;
use itonda_database::media::{
    find_assets_by_media_ids, insert_media_asset, upsert_media_external_id, MediaAssetInsert,
    MediaExternalIdUpsert,
};
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::{
    assets::types::AssetType,
    media::{
        models::{Asset, ExternalIdProvider, MediaExternalId},
        types::MediaType,
    },
    sources::youtube::client::YoutubeClient,
    storage::path::AppPaths,
    sync::{context::SyncContext, errors::SyncError, pipeline::SyncStep},
};

pub const MAX_AUDIO_DURATION_SECS: u64 = 5 * 60;

pub struct DownloadStep {
    pool: SqlitePool,
    paths: AppPaths,
    client: YoutubeClient,
}

impl DownloadStep {
    pub fn new(pool: SqlitePool, paths: AppPaths) -> Self {
        Self {
            pool,
            paths,
            client: YoutubeClient::new(),
        }
    }

    pub fn with_client(pool: SqlitePool, paths: AppPaths, client: YoutubeClient) -> Self {
        Self {
            pool,
            paths,
            client,
        }
    }
}

#[async_trait]
impl SyncStep for DownloadStep {
    fn name(&self) -> &'static str {
        "Download"
    }

    async fn execute(&self, context: &mut SyncContext) -> Result<(), SyncError> {
        let Some(media) = &mut context.media else {
            return Ok(());
        };

        let has_youtube_id = media
            .external_ids
            .iter()
            .any(|e| e.provider == ExternalIdProvider::Youtube);

        if !has_youtube_id && !context.force {
            return Ok(());
        }

        let media_uuid = Uuid::parse_str(&media.id)
            .map_err(|e| SyncError::Download(format!("Invalid media UUID {}: {e}", media.id)))?;

        let existing_assets = find_assets_by_media_ids(&self.pool, &[media.id.clone()]).await?;

        // 1. Video (e.g. trailer/gameplay/video clip)
        let has_video_asset = existing_assets
            .iter()
            .any(|a| a.asset_id == AssetType::Video.id() || a.asset_id == AssetType::Trailer.id());
        let video_rel_path = "video/video.mp4";
        let video_abs_path = self.paths.media_dir(media_uuid).join(video_rel_path);

        if !has_video_asset || !video_abs_path.exists() || context.force {
            let video_id = media
                .external_ids
                .iter()
                .find(|e| e.provider == ExternalIdProvider::Youtube)
                .map(|e| e.external_id.clone())
                .or_else(|| YoutubeClient::extract_video_id(&media.title));

            let video_target = if let Some(id) = video_id {
                Some(id)
            } else {
                let query = match media.media_type {
                    MediaType::Game => format!("{} game trailer", media.title),
                    MediaType::Movie => format!("{} official trailer", media.title),
                    MediaType::TvShow => format!("{} tv series trailer", media.title),
                };
                match self.client.search(&query, 1).await {
                    Ok(results) => results.into_iter().next().map(|r| r.id),
                    Err(err) => {
                        tracing::warn!("Failed to search YouTube video for '{}': {err}", media.title);
                        None
                    }
                }
            };

            if let Some(target) = video_target {
                tracing::info!(
                    "Downloading video for '{}' (ID: {}) to {}",
                    media.title,
                    target,
                    video_abs_path.display()
                );

                match self.client.download(&target, AssetType::Video, &video_abs_path).await {
                    Ok(_) => {
                        if !has_video_asset {
                            let inserted = insert_media_asset(
                                &self.pool,
                                MediaAssetInsert {
                                    media_id: media.id.clone(),
                                    asset_id: AssetType::Video.id(),
                                    path: video_rel_path.to_string(),
                                },
                            )
                            .await?;

                            media.assets.push(Asset {
                                id: inserted.id,
                                asset_type: AssetType::Video,
                                name: None,
                            });
                        }

                        if !media.external_ids.iter().any(|e| e.provider == ExternalIdProvider::Youtube) {
                            let _ = upsert_media_external_id(
                                &self.pool,
                                MediaExternalIdUpsert {
                                    media_id: media.id.clone(),
                                    provider: ExternalIdProvider::Youtube.as_str().to_string(),
                                    external_id: target.clone(),
                                },
                            )
                            .await;

                            media.external_ids.push(MediaExternalId {
                                provider: ExternalIdProvider::Youtube,
                                external_id: target,
                            });
                        }
                    }
                    Err(err) => {
                        tracing::warn!("Failed to download video for '{}': {err}", media.title);
                    }
                }
            }
        }

        // 2. Audio (e.g. soundtrack / theme converted to MP3)
        let existing_audio_asset = existing_assets
            .iter()
            .find(|a| a.asset_id == AssetType::Audio.id());
        let has_audio_asset = existing_audio_asset.is_some();
        let existing_file_exists = existing_audio_asset
            .map(|a| self.paths.media_dir(media_uuid).join(&a.path).exists())
            .unwrap_or(false);

        if !has_audio_asset || !existing_file_exists || context.force {
            let audio_query = format!("{} soundtrack", media.title);
            let mut search_results = match self
                .client
                .search_with_filter(&audio_query, 10, Some(MAX_AUDIO_DURATION_SECS))
                .await
            {
                Ok(results) => results,
                Err(err) => {
                    tracing::warn!("Failed to search YouTube audio for '{}': {err}", media.title);
                    Vec::new()
                }
            };

            if search_results.is_empty() {
                let theme_query = format!("{} theme", media.title);
                if let Ok(theme_results) = self
                    .client
                    .search_with_filter(&theme_query, 10, Some(MAX_AUDIO_DURATION_SECS))
                    .await
                {
                    search_results = theme_results;
                }
            }

            let audio_info = search_results
                .into_iter()
                .find(|info| info.duration.map_or(false, |d| d <= MAX_AUDIO_DURATION_SECS));

            if let Some(info) = audio_info {
                let track_title = info.title.clone();
                let sanitized_title = sanitize_filename(&track_title);
                let audio_rel_path = format!("audio/{sanitized_title}.mp3");
                let audio_abs_path = self.paths.media_dir(media_uuid).join(&audio_rel_path);

                tracing::info!(
                    "Downloading audio for '{}' (ID: {}, Title: '{}', Duration: {:?}s) to {}",
                    media.title,
                    info.id,
                    track_title,
                    info.duration,
                    audio_abs_path.display()
                );

                match self.client.download(&info.id, AssetType::Audio, &audio_abs_path).await {
                    Ok(_) => {
                        if let Some(existing) = existing_audio_asset {
                            if existing.path != audio_rel_path {
                                let old_abs_path = self.paths.media_dir(media_uuid).join(&existing.path);
                                if old_abs_path.exists() && old_abs_path != audio_abs_path {
                                    let _ = tokio::fs::remove_file(&old_abs_path).await;
                                }
                                let _ = sqlx::query("UPDATE media_assets SET path = ? WHERE id = ?")
                                    .bind(&audio_rel_path)
                                    .bind(&existing.id)
                                    .execute(&self.pool)
                                    .await;
                            }
                            if let Some(asset) = media.assets.iter_mut().find(|a| a.asset_type == AssetType::Audio) {
                                asset.name = Some(track_title.clone());
                            }
                        } else {
                            let inserted = insert_media_asset(
                                &self.pool,
                                MediaAssetInsert {
                                    media_id: media.id.clone(),
                                    asset_id: AssetType::Audio.id(),
                                    path: audio_rel_path.clone(),
                                },
                            )
                            .await?;

                            media.assets.push(Asset {
                                id: inserted.id,
                                asset_type: AssetType::Audio,
                                name: Some(track_title),
                            });
                        }
                    }
                    Err(err) => {
                        tracing::warn!("Failed to download audio for '{}': {err}", media.title);
                    }
                }
            }
        }

        Ok(())
    }
}

pub fn sanitize_filename(name: &str) -> String {
    let sanitized: String = name
        .chars()
        .map(|c| match c {
            '/' | '\\' | '?' | '%' | '*' | ':' | '|' | '"' | '<' | '>' => ' ',
            c if c.is_control() => ' ',
            c => c,
        })
        .collect();

    let words: Vec<&str> = sanitized.split_whitespace().collect();
    let result = words.join(" ");
    let trimmed = result.trim_matches(&[' ', '.', '-'][..]);
    if trimmed.is_empty() {
        "audio".to_string()
    } else {
        trimmed.chars().take(150).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use itonda_database::test_utils::setup_db;
    use tempfile::tempdir;

    #[tokio::test]
    async fn skips_when_media_missing() {
        let pool = setup_db().await;
        let temp = tempdir().unwrap();
        let paths = AppPaths {
            config_dir: temp.path().join("config"),
            data_dir: temp.path().join("data"),
        };

        let step = DownloadStep::new(pool, paths);
        let mut context = SyncContext {
            discovered: None,
            media: None,
            action: crate::sync::context::SyncAction::Unchanged,
            force: false,
        };

        let result = step.execute(&mut context).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn skips_when_already_downloaded_and_not_force() {
        let pool = setup_db().await;
        let temp = tempdir().unwrap();
        let paths = AppPaths {
            config_dir: temp.path().join("config"),
            data_dir: temp.path().join("data"),
        };

        let inserted_media = itonda_database::media::insert_media(
            &pool,
            itonda_database::media::MediaInsert::new("Test Game", "game", 1),
        )
        .await
        .unwrap();

        let media_uuid = Uuid::parse_str(&inserted_media.id).unwrap();
        let mut media = crate::media::models::Media::try_from(inserted_media).unwrap();
        media.external_ids = vec![MediaExternalId {
            provider: ExternalIdProvider::Youtube,
            external_id: "dQw4w9WgXcQ".to_string(),
        }];

        let mut context = SyncContext::from_media(media);
        context.force = false;

        let video_dir = paths.media_dir(media_uuid).join("video");
        let audio_dir = paths.media_dir(media_uuid).join("audio");
        tokio::fs::create_dir_all(&video_dir).await.unwrap();
        tokio::fs::create_dir_all(&audio_dir).await.unwrap();
        tokio::fs::write(video_dir.join("video.mp4"), b"fake video").await.unwrap();
        tokio::fs::write(audio_dir.join("audio.mp3"), b"fake audio").await.unwrap();

        insert_media_asset(
            &pool,
            MediaAssetInsert {
                media_id: media_uuid.to_string(),
                asset_id: AssetType::Video.id(),
                path: "video/video.mp4".to_string(),
            },
        )
        .await
        .unwrap();

        insert_media_asset(
            &pool,
            MediaAssetInsert {
                media_id: media_uuid.to_string(),
                asset_id: AssetType::Audio.id(),
                path: "audio/audio.mp3".to_string(),
            },
        )
        .await
        .unwrap();

        let step = DownloadStep::new(pool, paths);
        let result = step.execute(&mut context).await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_sanitize_filename() {
        assert_eq!(
            sanitize_filename("Kingdom Come: Deliverance OST - A World Unforgiving"),
            "Kingdom Come Deliverance OST - A World Unforgiving"
        );
        assert_eq!(
            sanitize_filename("Elden Ring / Main Theme? (Official)"),
            "Elden Ring Main Theme (Official)"
        );
        assert_eq!(sanitize_filename(""), "audio");
    }

    #[test]
    fn test_max_audio_duration_secs_constant() {
        // 5 minutes = 300 seconds
        assert_eq!(MAX_AUDIO_DURATION_SECS, 300);
    }
}
