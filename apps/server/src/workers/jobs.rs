use itonda_domain::media::models::MediaType;
use itonda_storefronts::models::StorefrontId;
use uuid::Uuid;

pub enum Job {
    Import(ImportJob),
    Sync(SyncJob),
}

pub struct ImportJob {
    pub id: Uuid,
    pub items: Vec<ImportItem>,
}

pub struct ImportItem {
    pub title: String,
    pub media_type: MediaType,
}

pub struct SyncJob {
    pub storefront: StorefrontId,
}
