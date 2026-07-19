use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct MediaRow {
    pub id: String,
    pub title: String,
    pub media_type: String,
}

#[derive(Debug)]
pub struct MediaInsert {
    pub title: String,
    pub media_type: String,
}
