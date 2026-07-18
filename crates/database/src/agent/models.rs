#[derive(Debug)]
pub struct AgentsRow {
    pub id: String,
    pub name: String,
    pub hostname: Option<String>,
    pub platform: Option<String>,
    pub agent_version: Option<String>,
    pub last_seen_at: Option<i64>,
    pub created_at: i64,
}

#[derive(Debug)]
pub struct AgentsInsert {
    pub id: String,
    pub name: String,
    pub hostname: String,
    pub platform: String,
    pub agent_version: String,
}

#[derive(Debug)]
pub struct AgentConnectionsRow {
    pub id: String,
    pub agent_id: String,
    pub connected_at: i64,
    pub disconnected_at: Option<i64>,
    pub ip_address: Option<String>,
}

pub struct AgentConnectionsInsert {
    pub id: String,
    pub agent_id: String,
    pub ip_address: Option<String>,
}
