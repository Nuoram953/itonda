use itonda_database::media as MediaQueries;
use sqlx::SqlitePool;

use crate::media::{errors::MediaError, models::Media};

pub async fn get_all_media(pool: &SqlitePool) -> Result<Vec<Media>, MediaError> {
    let rows = MediaQueries::find_all(pool).await?;

    let media = rows
        .into_iter()
        .map(Media::try_from)
        .collect::<Result<Vec<_>, _>>();

    Ok(media.unwrap())
}
