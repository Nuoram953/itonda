use crate::media::{
    errors::MediaError,
    models::{Media, MediaType},
};

use tracing::instrument;

use itonda_database::media::{MediaInsert, insert_media};
use sqlx::SqlitePool;

#[derive(Debug)]
pub struct MediaImport {
    pub title: String,
    pub media_type: MediaType,
    pub year: Option<i64>,
}

#[instrument(skip(pool))]
pub async fn import(pool: &SqlitePool, input: MediaImport) -> Result<Media, MediaError> {
    if input.title.trim().is_empty() {
        return Err(MediaError::InvalidTitle);
    }

    let row = insert_media(
        pool,
        MediaInsert {
            title: input.title,
            media_type: input.media_type.as_str().to_string(),
            year: input.year,
        },
    )
    .await?;

    let media = Media::try_from(row)?;

    Ok(media)
}
