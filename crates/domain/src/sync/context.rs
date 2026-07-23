use crate::media::models::{Media, MediaSource, MediaType};

pub struct SyncContext {
    pub discovered: DiscoveredMedia,
    pub media: Option<Media>,
}

pub struct DiscoveredMedia {
    pub external_id: String,
    pub source: MediaSource,
    pub media_type: MediaType,
    pub title: String,
}
