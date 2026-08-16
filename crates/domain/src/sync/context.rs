use itonda_database::models::UpsertAction;

use crate::media::{discovered::DiscoveredMedia, models::Media};

#[derive(Debug)]
pub struct SyncContext {
    pub discovered: Option<DiscoveredMedia>,
    pub media: Option<Media>,
    pub action: SyncAction,
    pub force: bool,
}

impl SyncContext {
    pub fn new(discovered: DiscoveredMedia, force: bool) -> Self {
        Self {
            discovered: Some(discovered),
            media: None,
            action: SyncAction::Unchanged,
            force,
        }
    }

    pub fn from_media(media: Media) -> Self {
        Self {
            discovered: None,
            media: Some(media),
            action: SyncAction::Unchanged,
            force: false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyncAction {
    Created,
    Updated,
    Unchanged,
}

impl SyncAction {
    pub fn merge(&mut self, other: SyncAction) {
        if other.priority() > self.priority() {
            *self = other;
        }
    }

    fn priority(&self) -> u8 {
        match self {
            Self::Unchanged => 0,
            Self::Updated => 1,
            Self::Created => 2,
        }
    }
}

impl From<UpsertAction> for SyncAction {
    fn from(action: UpsertAction) -> Self {
        match action {
            UpsertAction::Created => Self::Created,
            UpsertAction::Updated => Self::Updated,
            UpsertAction::Unchanged => Self::Unchanged,
        }
    }
}
