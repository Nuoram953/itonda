use serde::Serialize;

#[derive(Debug, Serialize, Clone, PartialEq)]
pub enum ImportEvent {
    Started,

    Progress { message: String, progress: u8 },

    Completed,
}
