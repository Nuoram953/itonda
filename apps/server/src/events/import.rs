use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum ImportEvent {
    Started {
        job_id: Uuid,
    },

    Progress {
        job_id: Uuid,
        step: ImportStep,
        message: String,
        progress: u8,
    },

    Completed {
        job_id: Uuid,
    },

    Failed {
        job_id: Uuid,
        error: String,
    },
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum ImportStep {
    Scanning,
    Metadata,
    Artwork,
    Trailer,
    Audio,
    Saving,
}
