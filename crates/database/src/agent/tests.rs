use crate::{
    agent::{
        AgentConnectionsInsert, AgentsInsert, disconnect_agent_connection, insert_agent_connection,
        upsert_agent,
    },
    test_utils::setup_db,
};

#[tokio::test]
async fn upsert_agent_creates_agent() {
    let pool = setup_db().await;

    let row = upsert_agent(
        &pool,
        AgentsInsert {
            id: "agent-1".to_string(),
            name: "Gaming PC".to_string(),
            hostname: "desktop".to_string(),
            platform: "linux".to_string(),
            agent_version: "1.0.0".to_string(),
        },
    )
    .await
    .unwrap();

    assert_eq!(row.id, "agent-1");
    assert_eq!(row.name, "Gaming PC");
    assert_eq!(row.hostname, Some("desktop".to_string()));
    assert_eq!(row.platform, Some("linux".to_string()));
    assert_eq!(row.agent_version, Some("1.0.0".to_string()));
}

#[tokio::test]
async fn upsert_agent_updates_existing_agent() {
    let pool = setup_db().await;

    upsert_agent(
        &pool,
        AgentsInsert {
            id: "agent-1".to_string(),
            name: "Gaming PC".to_string(),
            hostname: "desktop".to_string(),
            platform: "linux".to_string(),
            agent_version: "1.0.0".to_string(),
        },
    )
    .await
    .unwrap();

    let row = upsert_agent(
        &pool,
        AgentsInsert {
            id: "agent-1".to_string(),
            name: "Living Room PC".to_string(),
            hostname: "media-pc".to_string(),
            platform: "windows".to_string(),
            agent_version: "2.0.0".to_string(),
        },
    )
    .await
    .unwrap();

    assert_eq!(row.id, "agent-1");
    assert_eq!(row.name, "Living Room PC");
    assert_eq!(row.hostname, Some("media-pc".to_string()));
    assert_eq!(row.platform, Some("windows".to_string()));
    assert_eq!(row.agent_version, Some("2.0.0".to_string()));
}

#[tokio::test]
async fn insert_agent_connection_creates_connection() {
    let pool = setup_db().await;

    upsert_agent(
        &pool,
        AgentsInsert {
            id: "agent-1".to_string(),
            name: "Gaming PC".to_string(),
            hostname: "desktop".to_string(),
            platform: "linux".to_string(),
            agent_version: "1.0.0".to_string(),
        },
    )
    .await
    .unwrap();

    let row = insert_agent_connection(
        &pool,
        AgentConnectionsInsert {
            id: "connection-1".to_string(),
            agent_id: "agent-1".to_string(),
            ip_address: Some("192.168.1.100".to_string()),
        },
    )
    .await
    .unwrap();

    assert_eq!(row.id, "connection-1");
    assert_eq!(row.agent_id, "agent-1");
    assert_eq!(row.ip_address, Some("192.168.1.100".to_string()));
    assert!(row.disconnected_at.is_none());
}

#[tokio::test]
async fn disconnect_agent_connection_sets_disconnected_at() {
    let pool = setup_db().await;

    upsert_agent(
        &pool,
        AgentsInsert {
            id: "agent-1".to_string(),
            name: "Gaming PC".to_string(),
            hostname: "desktop".to_string(),
            platform: "linux".to_string(),
            agent_version: "1.0.0".to_string(),
        },
    )
    .await
    .unwrap();

    insert_agent_connection(
        &pool,
        AgentConnectionsInsert {
            id: "connection-1".to_string(),
            agent_id: "agent-1".to_string(),
            ip_address: Some("192.168.1.100".to_string()),
        },
    )
    .await
    .unwrap();

    let result = disconnect_agent_connection(&pool, "agent-1".to_string())
        .await
        .unwrap();

    assert_eq!(result.rows_affected(), 1);

    let row = sqlx::query(
        r#"
    SELECT disconnected_at
    FROM agent_connections
    WHERE id = ?
    "#,
    )
    .bind("connection-1")
    .fetch_one(&pool)
    .await
    .unwrap();

    use sqlx::Row;

    let disconnected_at: Option<i64> = row.get("disconnected_at");
    assert!(disconnected_at.is_some());
}

#[tokio::test]
async fn disconnect_agent_connection_returns_zero_rows_when_agent_not_connected() {
    let pool = setup_db().await;

    let result = disconnect_agent_connection(&pool, "missing-agent".to_string())
        .await
        .unwrap();

    assert_eq!(result.rows_affected(), 0);
}
