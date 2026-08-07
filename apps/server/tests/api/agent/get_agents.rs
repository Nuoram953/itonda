use axum::{body::Body, http};
use http::{Request, StatusCode};
use itonda_database::agent::{
    disconnect_agent_connection, insert_agent_connection, upsert_agent, AgentConnectionsInsert,
    AgentsInsert,
};
use itonda_server::api::agents::schemas::GetAgentsResponse;
use tower::ServiceExt;
use uuid::Uuid;

use crate::common::{app::test_app, response::json};

#[tokio::test]
async fn get_agents_returns_connected_agents() {
    let app = test_app().await;

    let response = app
        .router
        .oneshot(
            Request::builder()
                .uri("/agents")
                .method("GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body: GetAgentsResponse = json(response).await;
    assert_eq!(body.agents.len(), 1);
    assert_eq!(body.agents[0].name, "test-agent");
    assert!(body.agents[0].is_connected);
}

#[tokio::test]
async fn get_agents_returns_empty_list_when_no_agents() {
    let app = test_app().await;

    sqlx::query("DELETE FROM agent_connections")
        .execute(&app.db)
        .await
        .unwrap();
    sqlx::query("DELETE FROM agents")
        .execute(&app.db)
        .await
        .unwrap();

    let response = app
        .router
        .oneshot(
            Request::builder()
                .uri("/agents")
                .method("GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body: GetAgentsResponse = json(response).await;
    assert!(body.agents.is_empty());
}

#[tokio::test]
async fn get_agents_returns_multiple_agents_with_status() {
    let app = test_app().await;

    // Insert a 2nd active agent
    let agent2_id = Uuid::new_v4().to_string();
    upsert_agent(
        &app.db,
        AgentsInsert {
            id: agent2_id.clone(),
            name: "agent-2".into(),
            hostname: "host-2".into(),
            platform: "windows".into(),
            agent_version: "1.0.0".into(),
        },
    )
    .await
    .unwrap();

    insert_agent_connection(
        &app.db,
        AgentConnectionsInsert {
            id: Uuid::new_v4().to_string(),
            agent_id: agent2_id.clone(),
            ip_address: Some("192.168.1.10".into()),
        },
    )
    .await
    .unwrap();

    // Insert a 3rd agent that gets disconnected
    let agent3_id = Uuid::new_v4().to_string();
    upsert_agent(
        &app.db,
        AgentsInsert {
            id: agent3_id.clone(),
            name: "agent-3-offline".into(),
            hostname: "host-3".into(),
            platform: "macos".into(),
            agent_version: "1.0.0".into(),
        },
    )
    .await
    .unwrap();

    insert_agent_connection(
        &app.db,
        AgentConnectionsInsert {
            id: Uuid::new_v4().to_string(),
            agent_id: agent3_id.clone(),
            ip_address: Some("192.168.1.11".into()),
        },
    )
    .await
    .unwrap();

    disconnect_agent_connection(&app.db, agent3_id.clone())
        .await
        .unwrap();

    let response = app
        .router
        .oneshot(
            Request::builder()
                .uri("/agents")
                .method("GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body: GetAgentsResponse = json(response).await;
    // Should return 2 active connected agents (test-agent & agent-2)
    assert_eq!(body.agents.len(), 2);
    assert!(body.agents.iter().all(|a| a.is_connected));
}
