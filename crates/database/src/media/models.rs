use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct MediaRow {
    pub id: String,
    pub title: String,
    pub media_type: String,
}

#[derive(Debug, Serialize)]
pub struct MediaGameStorefrontRow {
    pub media_id: String,
    pub storefront_id: String,
    pub internal_id: String,
}

#[derive(Debug)]
pub struct MediaInsert {
    pub title: String,
    pub media_type: String,
}

#[derive(Debug)]
pub struct MediaGameStorefrontUpsert {
    pub media_id: String,
    pub storefront_id: u32,
    pub internal_id: String,
}
