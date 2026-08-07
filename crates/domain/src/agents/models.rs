use itonda_database::agent::AgentWithStatusRow;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::agents::errors::AgentsError;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Agent {
    pub id: String,
    pub name: String,
    pub hostname: Option<String>,
    pub platform: Option<String>,
    pub agent_version: Option<String>,
    pub last_seen_at: Option<i64>,
    pub created_at: i64,
    pub is_connected: bool,
    pub connected_at: Option<i64>,
    pub ip_address: Option<String>,
}

impl TryFrom<AgentWithStatusRow> for Agent {
    type Error = AgentsError;

    fn try_from(row: AgentWithStatusRow) -> Result<Self, Self::Error> {
        Ok(Self {
            id: row.id,
            name: row.name,
            hostname: row.hostname,
            platform: row.platform,
            agent_version: row.agent_version,
            last_seen_at: row.last_seen_at,
            created_at: row.created_at,
            is_connected: row.connected_at.is_some(),
            connected_at: row.connected_at,
            ip_address: row.ip_address,
        })
    }
}
