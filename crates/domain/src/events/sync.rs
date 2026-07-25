use serde::Serialize;

#[derive(Debug, Serialize, Clone, PartialEq)]
#[serde(tag = "type", content = "payload")]
pub enum SyncEvent {
    Started,

    MediaFound { title: String },

    MediaSynced { media_id: String },

    Completed,
}
