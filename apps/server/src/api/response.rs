use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct JobResponse {
    pub job_id: String,
    pub status: JobStatus,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum JobStatus {
    Queued,
    Running,
    Completed,
    Failed,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CommandResponse {
    pub id: String,
    pub command: String,
    pub status: CommandStatus,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum CommandStatus {
    Accepted,
    Sent,
}
