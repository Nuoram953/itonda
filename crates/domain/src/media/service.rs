use std::collections::HashMap;

use itonda_database::{
    media::{
        self as MediaQueries, MediaGameDetailsRow, MediaGameDetailsUpsert,
        MediaLaunchSessionInsert, MediaStorefrontUpsert,
    },
    models::UpsertResult,
};
use sqlx::SqlitePool;

use crate::{
    media::{
        errors::MediaError,
        models::{
            Asset, Launch, Media, MediaDetails, MediaInstallation, MediaStorefront, PaginatedMedia,
        },
        types::{MediaLaunchType, MediaSortField, MediaStatus, MediaType, SortOrder},
    },
    storefronts::models::StorefrontId,
    utils::datetime::parse_timestamp,
};

pub async fn get_media_by_id(pool: &SqlitePool, id: String) -> Result<Media, MediaError> {
    let row = MediaQueries::find_media_by_id(pool, id).await?;

    let mut media = Media::try_from(row)?;

    let assets = MediaQueries::find_assets_by_media_ids(pool, &[media.id.clone()]).await?;

    media.assets = assets
        .into_iter()
        .map(Asset::try_from)
        .collect::<Result<Vec<_>, _>>()?;

    let storefronts =
        MediaQueries::find_storefronts_by_media_ids(pool, &[media.id.clone()]).await?;

    media.storefronts = storefronts
        .into_iter()
        .map(MediaStorefront::try_from)
        .collect::<Result<Vec<_>, _>>()?;

    let installations =
        MediaQueries::find_installations_by_media_ids(pool, &[media.id.clone()]).await?;

    media.installations = installations
        .into_iter()
        .map(MediaInstallation::try_from)
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
    let storefronts = MediaQueries::find_storefronts_by_media_ids(pool, &ids).await?;
    let installations = MediaQueries::find_installations_by_media_ids(pool, &ids).await?;
    let launches = MediaQueries::find_media_launches_by_media_ids(pool, &ids).await?;

    let assets_by_media = assets.into_iter().fold(HashMap::new(), |mut map, asset| {
        map.entry(asset.media_id.clone())
            .or_insert_with(Vec::new)
            .push(asset);
        map
    });

    let storefronts_by_media = storefronts.into_iter().fold(HashMap::new(), |mut map, sf| {
        map.entry(sf.media_id.clone())
            .or_insert_with(Vec::new)
            .push(sf);
        map
    });

    let installations_by_media = installations
        .into_iter()
        .fold(HashMap::new(), |mut map, inst| {
            map.entry(inst.media_id.clone())
                .or_insert_with(Vec::new)
                .push(inst);
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

        media.storefronts = storefronts_by_media
            .get(&media.id)
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .map(MediaStorefront::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        media.installations = installations_by_media
            .get(&media.id)
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .map(MediaInstallation::try_from)
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

pub async fn recalculate_media_game_details(
    pool: &SqlitePool,
    media_id: &str,
) -> Result<UpsertResult<MediaGameDetailsRow>, MediaError> {
    let storefronts = MediaQueries::find_storefronts_by_media_id(pool, media_id).await?;
    let launches = MediaQueries::find_media_launches_by_media_id(pool, media_id).await?;
    let sessions = MediaQueries::find_launch_sessions_by_media_id(pool, media_id).await?;

    let mut total_playtime_minutes = 0i64;
    let mut latest_last_played_at: Option<i64> = None;

    for sf in &storefronts {
        if let Some(minutes) = sf.playtime_minutes {
            total_playtime_minutes += minutes;
        }
        if let Some(ts) = sf.last_played_at {
            latest_last_played_at = Some(latest_last_played_at.map_or(ts, |curr| curr.max(ts)));
        }
    }

    let storefront_launch_ids: std::collections::HashSet<&str> = if storefronts.is_empty() {
        std::collections::HashSet::new()
    } else {
        launches
            .iter()
            .filter(|l| {
                let lt = l.launch_type.to_lowercase();
                lt == MediaLaunchType::Storefront.as_str()
            })
            .map(|l| l.id.as_str())
            .collect()
    };

    for s in &sessions {
        if !storefront_launch_ids.contains(s.launch_id.as_str()) {
            let dur: i64 = s.duration_seconds.parse().unwrap_or(0);
            total_playtime_minutes += dur / 60;
        }

        let session_ts =
            parse_timestamp(&s.completed_at).or_else(|| parse_timestamp(&s.started_at));
        if let Some(ts) = session_ts {
            latest_last_played_at = Some(latest_last_played_at.map_or(ts, |curr| curr.max(ts)));
        }
    }

    let res = MediaQueries::upsert_media_game_details(
        pool,
        MediaGameDetailsUpsert {
            media_id: media_id.to_string(),
            playtime_minutes: Some(total_playtime_minutes),
            last_played_at: latest_last_played_at,
        },
    )
    .await?;

    Ok(res)
}

pub async fn update_playtime(
    pool: &SqlitePool,
    session: MediaLaunchSessionInsert,
) -> Result<(), MediaError> {
    let launch = MediaQueries::find_media_launch_by_id(pool, session.launch_id.clone()).await?;

    let session_duration_seconds: i64 = session.duration_seconds.parse().unwrap_or(0);
    let session_minutes = session_duration_seconds / 60;

    let last_played_at = parse_timestamp(&session.completed_at)
        .or_else(|| parse_timestamp(&session.started_at))
        .unwrap_or_else(|| chrono::Utc::now().timestamp());

    let lt = launch.launch_type.to_lowercase();
    let is_storefront = lt == MediaLaunchType::Storefront.as_str();

    if is_storefront {
        let storefronts =
            MediaQueries::find_storefronts_by_media_id(pool, &launch.media_id).await?;

        let matched = storefronts
            .iter()
            .find(|sf| {
                if let Ok(sf_id) = StorefrontId::try_from(launch.name.as_str()) {
                    sf.storefront_id == sf_id.as_str()
                } else if let Ok(sf_id) = StorefrontId::try_from(launch.launch_type.as_str()) {
                    sf.storefront_id == sf_id.as_str()
                } else {
                    false
                }
            })
            .or_else(|| {
                if storefronts.len() == 1 {
                    storefronts.first()
                } else {
                    None
                }
            });

        if let Some(sf) = matched {
            let current_playtime = sf.playtime_minutes.unwrap_or(0);
            let new_playtime = current_playtime + session_minutes;

            MediaQueries::upsert_media_storefront(
                pool,
                MediaStorefrontUpsert {
                    media_id: sf.media_id.clone(),
                    storefront_id: sf.storefront_id.clone(),
                    external_id: sf.external_id.clone(),
                    playtime_minutes: Some(new_playtime),
                    last_played_at: Some(last_played_at),
                },
            )
            .await?;
        }
    }

    MediaQueries::insert_media_launch_session(pool, session).await?;

    recalculate_media_game_details(pool, &launch.media_id).await?;

    Ok(())
}
