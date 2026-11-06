use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "type", content = "payload")]
pub enum SyncEvent {
    Started,

    MediaFound {
        title: String,
    },

    MediaSynced {
        media_id: String,
    },

    StepProgress {
        #[serde(skip_serializing_if = "Option::is_none")]
        media_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        title: Option<String>,
        step: String,
        message: String,
    },

    MediaSyncFailed {
        media_id: String,
        error: String,
    },

    Completed,
}
