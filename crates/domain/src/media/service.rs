use std::collections::HashMap;

use itonda_database::media as MediaQueries;
use sqlx::SqlitePool;

use crate::media::{
    errors::MediaError,
    models::{Asset, Media},
};

pub async fn get_all_media(pool: &SqlitePool) -> Result<Vec<Media>, MediaError> {
    let rows = MediaQueries::find_all(pool).await?;

    let ids = rows.iter().map(|row| row.id.clone()).collect::<Vec<_>>();

    let assets = MediaQueries::find_assets_by_media_ids(pool, &ids).await?;

    let assets_by_media = assets.into_iter().fold(HashMap::new(), |mut map, asset| {
        map.entry(asset.media_id.clone())
            .or_insert_with(Vec::new)
            .push(asset);
        map
    });

    rows.into_iter()
        .map(|row| {
            Media::try_from(row).and_then(|mut media| {
                media.assets = assets_by_media
                    .get(&media.id)
                    .cloned()
                    .unwrap_or_default()
                    .into_iter()
                    .map(Asset::try_from)
                    .collect::<Result<Vec<_>, _>>()?;

                Ok(media)
            })
        })
        .collect()
}
