use serde::Serialize;

#[derive(Debug, Serialize, Clone, PartialEq)]
pub enum SyncEvent {
    Started,

    MediaFound { title: String },

    MediaSynced { media_id: String },

    Completed,
}
