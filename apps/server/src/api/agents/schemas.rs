use itonda_domain::{agents::models::Agent, protocol::DirectoryEntry};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct GetAgentsResponse {
    pub agents: Vec<Agent>,
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct ListAgentFilesQuery {
    pub agent_id: Option<String>,
    pub path: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AgentDirectoryListingResponse {
    pub path: String,
    pub parent: Option<String>,
    pub entries: Vec<DirectoryEntry>,
    pub error: Option<String>,
}
