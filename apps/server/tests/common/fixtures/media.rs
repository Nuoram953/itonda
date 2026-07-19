use itonda_database::media::{MediaInsert, MediaRow, insert_media};
use sqlx::SqlitePool;

pub struct MediaFixture {
    pub title: String,
    pub media_type: String,
}

impl Default for MediaFixture {
    fn default() -> Self {
        Self {
            title: "Test Game".into(),
            media_type: "game".into(),
        }
    }
}

impl MediaFixture {
    pub async fn insert(self, db: &SqlitePool) -> MediaRow {
        insert_media(
            db,
            MediaInsert {
                title: self.title,
                media_type: self.media_type,
            },
        )
        .await
        .unwrap()
    }
}
