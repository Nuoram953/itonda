use itonda_domain::media::models::MediaType;
use uuid::Uuid;

pub enum Job {
    Import(ImportJob),
}

pub struct ImportJob {
    pub id: Uuid,
    pub items: Vec<ImportItem>,
}

pub struct ImportItem {
    pub title: String,
    pub media_type: MediaType,
    pub year: Option<i64>,
}
