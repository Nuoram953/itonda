use crate::media::models::{DiscoveredMedia, Media};

pub struct SyncContext {
    pub discovered: DiscoveredMedia,
    pub media: Option<Media>,
    pub action: SyncAction,
}

impl SyncContext {
    pub fn new(discovered: DiscoveredMedia) -> Self {
        Self {
            discovered,
            media: None,
            action: SyncAction::Unchanged,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyncAction {
    Created,
    Updated,
    Unchanged,
}
