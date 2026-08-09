use std::collections::HashMap;

use itonda_database::media as MediaQueries;
use sqlx::SqlitePool;

use crate::media::{
    errors::MediaError,
    models::{Asset, Launch, Media, MediaDetails, PaginatedMedia},
    types::{MediaSortField, MediaStatus, MediaType, SortOrder},
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

#[derive(Debug, Clone, Default)]
pub struct MediaSearchQuery<'a> {
    pub media_type: Option<MediaType>,
    pub search: Option<&'a str>,
    pub status: Option<MediaStatus>,
    pub storefront: Option<&'a str>,
    pub sort_by: Option<MediaSortField>,
    pub sort_order: Option<SortOrder>,
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

pub async fn get_paginated_media(
    pool: &SqlitePool,
    query: MediaSearchQuery<'_>,
) -> Result<PaginatedMedia, MediaError> {
    let page = query.page.unwrap_or(1).max(1);
    let limit = query.limit.unwrap_or(24).max(1);

    let type_str = query.media_type.as_ref().map(|t| t.as_str());

    let db_sort_by = query.sort_by.map(|sb| match sb {
        MediaSortField::Title => MediaQueries::DbMediaSortField::Title,
        MediaSortField::LastPlayedAt => MediaQueries::DbMediaSortField::LastPlayedAt,
    });

    let db_sort_order = query.sort_order.map(|so| match so {
        SortOrder::Asc => MediaQueries::DbSortOrder::Asc,
        SortOrder::Desc => MediaQueries::DbSortOrder::Desc,
    });

    let db_options = MediaQueries::DbMediaFilterOptions {
        media_type: type_str,
        search: query.search,
        status_id: query.status.map(|s| s.id()),
        storefront_id: query.storefront,
        sort_by: db_sort_by,
        sort_order: db_sort_order,
        page,
        limit,
    };

    let result = MediaQueries::find_paginated(pool, db_options).await?;

    let ids = result
        .items
        .iter()
        .map(|row| row.id.clone())
        .collect::<Vec<_>>();

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

    let mut medias = Vec::with_capacity(result.items.len());

    for row in result.items {
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

    let total_pages = if result.total == 0 {
        1
    } else {
        ((result.total as f64) / (limit as f64)).ceil() as u32
    };

    let has_next = page < total_pages;

    Ok(PaginatedMedia {
        items: medias,
        total: result.total,
        page,
        limit,
        total_pages,
        has_next,
    })
}

pub async fn get_all_media(
    pool: &SqlitePool,
    media_type: Option<MediaType>,
) -> Result<Vec<Media>, MediaError> {
    let paginated = get_paginated_media(
        pool,
        MediaSearchQuery {
            media_type,
            page: Some(1),
            limit: Some(u32::MAX),
            ..Default::default()
        },
    )
    .await?;

    Ok(paginated.items)
}

pub async fn update_status(
    pool: &SqlitePool,
    media_id: String,
    status_id: MediaStatus,
) -> Result<(), MediaError> {
    MediaQueries::update_media_status(pool, &media_id, status_id.id()).await?;

    Ok(())
}
