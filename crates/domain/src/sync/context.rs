use crate::media::models::{DiscoveredMedia, Media};

pub struct SyncContext {
    pub discovered: DiscoveredMedia,
    pub media: Option<Media>,
}

impl SyncContext {
    pub fn new(discovered: DiscoveredMedia) -> Self {
        Self {
            discovered,
            media: None,
        }
    }
}
