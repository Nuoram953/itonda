use serde::Serialize;
use uuid::Uuid;

use super::{ImportEvent, SyncEvent};

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct JobEvent {
    pub job_id: Uuid,
    pub job_type: JobType,
    pub event: JobEventType,
}

#[derive(Debug, Serialize, Clone, PartialEq)]
#[serde(tag = "type", content = "payload")]
pub enum JobType {
    Import,
    Sync,
}

#[derive(Debug, Serialize, Clone, PartialEq)]
#[serde(tag = "type", content = "payload")]
pub enum JobEventType {
    Started,

    Progress { current: usize, total: usize },

    Completed,

    Failed { error: String },

    Import(ImportEvent),

    Sync(SyncEvent),
}
