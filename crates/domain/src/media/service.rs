use std::collections::HashMap;

use itonda_database::media as MediaQueries;
use sqlx::SqlitePool;

use crate::media::{
    errors::MediaError,
    models::{Asset, Launch, Media, MediaDetails},
    types::{MediaStatus, MediaType},
};

pub async fn get_media_by_id(pool: &SqlitePool, id: String) -> Result<Media, MediaError> {
    let row = MediaQueries::find_media_by_id(pool, id).await?;

    let mut media = Media::try_from(row)?;

    let assets = MediaQueries::find_assets_by_media_ids(pool, &[media.id.clone()]).await?;

    media.assets = assets
        .into_iter()
        .map(Asset::try_from)
        .collect::<Result<Vec<_>, _>>()?;

    let launches = MediaQueries::find_media_launches_by_media_id(pool, &media.id).await?;

    media.launches = launches.into_iter().map(Launch::from).collect();

    media.details = match media.media_type {
        MediaType::Game => {
            let details = MediaQueries::find_game_details(pool, &media.id).await?;

            details.map(|details| MediaDetails::Game(details.into()))
        }

        MediaType::Movie => None,

        MediaType::TvShow => None,
    };

    Ok(media)
}

pub async fn get_all_media(
    pool: &SqlitePool,
    media_type: Option<MediaType>,
) -> Result<Vec<Media>, MediaError> {
    let type_str = media_type.as_ref().map(|t| t.as_str());
    let rows = MediaQueries::find_all(pool, type_str).await?;

    let ids = rows.iter().map(|row| row.id.clone()).collect::<Vec<_>>();

    let assets = MediaQueries::find_assets_by_media_ids(pool, &ids).await?;
    let launches = MediaQueries::find_media_launches_by_media_ids(pool, &ids).await?;

    let assets_by_media = assets.into_iter().fold(HashMap::new(), |mut map, asset| {
        map.entry(asset.media_id.clone())
            .or_insert_with(Vec::new)
            .push(asset);
        map
    });

    let launches_by_media = launches
        .into_iter()
        .fold(HashMap::new(), |mut map, launch| {
            map.entry(launch.media_id.clone())
                .or_insert_with(Vec::new)
                .push(launch);
            map
        });

    let mut medias = Vec::with_capacity(rows.len());

    for row in rows {
        let mut media = Media::try_from(row)?;

        media.assets = assets_by_media
            .get(&media.id)
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .map(Asset::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        media.launches = launches_by_media
            .get(&media.id)
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .map(Launch::from)
            .collect();

        media.details = match media.media_type {
            MediaType::Game => {
                let details = MediaQueries::find_game_details(pool, &media.id).await?;

                details.map(|details| MediaDetails::Game(details.into()))
            }

            MediaType::Movie => None,

            MediaType::TvShow => None,
        };

        medias.push(media);
    }

    Ok(medias)
}

pub async fn update_status(
    pool: &SqlitePool,
    media_id: String,
    status_id: MediaStatus,
) -> Result<(), MediaError> {
    MediaQueries::update_media_status(pool, &media_id, status_id.id()).await?;

    Ok(())
}
