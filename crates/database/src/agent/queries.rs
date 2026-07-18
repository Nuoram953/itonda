use sqlx::{SqlitePool, sqlite::SqliteQueryResult};

use crate::{
    agent::{AgentConnectionsInsert, AgentConnectionsRow, AgentsInsert, AgentsRow},
    error::DatabaseError,
};

pub async fn upsert_agent(
    pool: &SqlitePool,
    agent: AgentsInsert,
) -> Result<AgentsRow, DatabaseError> {
    sqlx::query_as!(
        AgentsRow,
        r#"
        INSERT INTO agents (
            id,
            name,
            hostname,
            platform,
            agent_version
        )
        VALUES (?, ?, ?, ?, ?)
        ON CONFLICT(id) DO UPDATE SET
            name = excluded.name,
            hostname = excluded.hostname,
            platform = excluded.platform,
            agent_version = excluded.agent_version,
            last_seen_at = unixepoch()
        RETURNING
            id,
            name,
            hostname,
            platform,
            agent_version,
            last_seen_at,
            created_at
        "#,
        agent.id,
        agent.name,
        agent.hostname,
        agent.platform,
        agent.agent_version
    )
    .fetch_one(pool)
    .await
    .map_err(DatabaseError::from)
}

pub async fn insert_agent_connection(
    pool: &SqlitePool,
    connection: AgentConnectionsInsert,
) -> Result<AgentConnectionsRow, DatabaseError> {
    sqlx::query_as!(
        AgentConnectionsRow,
        r#"
        INSERT INTO agent_connections (
            id,
            agent_id,
            ip_address
        )
        VALUES (?, ?, ?)
        RETURNING
            id,
            agent_id,
            connected_at,
            disconnected_at,
            ip_address
        "#,
        connection.id,
        connection.agent_id,
        connection.ip_address
    )
    .fetch_one(pool)
    .await
    .map_err(DatabaseError::from)
}

pub async fn disconnect_agent_connection(
    pool: &SqlitePool,
    agent_id: String,
) -> Result<SqliteQueryResult, DatabaseError> {
    sqlx::query!(
        r#"
        UPDATE agent_connections SET disconnected_at=unixepoch() WHERE agent_id=?;
        "#,
        agent_id
    )
    .execute(pool)
    .await
    .map_err(DatabaseError::from)
}
