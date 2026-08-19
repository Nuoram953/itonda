use axum::Router;
use itonda_database::{
    agent::{AgentConnectionsInsert, AgentsInsert, insert_agent_connection, upsert_agent},
    test_utils::setup_db,
};
use itonda_domain::{
    events::EventBus, protocol::ServerToAgentMessage, store::toml::TomlCodec,
    storefronts::registry::StorefrontRegistry,
};
use itonda_server::{
    api,
    config::{app::AppConfigManager, secrets::SecretsManager, settings::SettingsManager},
    state::AppState,
    websocket::AgentManager,
    workers::jobs::Job,
};
use sqlx::SqlitePool;
use tempfile::tempdir;
use tokio::sync::mpsc::Receiver;
use uuid::Uuid;

pub struct TestApp {
    pub router: Router,
    pub jobs: Receiver<Job>,
    pub db: SqlitePool,
    pub agent_messages: Receiver<ServerToAgentMessage>,
    pub _temp: tempfile::TempDir,
}

pub async fn test_app() -> TestApp {
    let db = setup_db().await;

    let temp = tempdir().unwrap();

    let settings = SettingsManager::load(temp.path().join("settings.toml"), TomlCodec).unwrap();

    let config = AppConfigManager::load(temp.path().join("config.toml"), TomlCodec).unwrap();

    let secrets = SecretsManager::load(temp.path().join("secrets.toml"), TomlCodec).unwrap();

    let (jobs, receiver) = tokio::sync::mpsc::channel(100);

    let agent_manager = AgentManager::new();

    let (agent_tx, agent_rx) = tokio::sync::mpsc::channel(32);

    let agent_id = create_agent(&db).await;

    agent_manager.register(agent_id, agent_tx).await;

    let state = AppState {
        db: db.clone(),
        jobs,
        events: EventBus::new(),
        settings,
        config,
        secrets,
        storefronts: StorefrontRegistry::new(),
        agent_manager,
    };

    TestApp {
        router: api::router().with_state(state),
        jobs: receiver,
        agent_messages: agent_rx,
        db,
        _temp: temp,
    }
}

async fn create_agent(pool: &SqlitePool) -> String {
    let agent_id = Uuid::new_v4().to_string();

    upsert_agent(
        pool,
        AgentsInsert {
            id: agent_id.clone(),
            name: "test-agent".into(),
            hostname: "test-agent".into(),
            platform: "linux".into(),
            agent_version: "0.0.0".into(),
        },
    )
    .await
    .unwrap();

    insert_agent_connection(
        pool,
        AgentConnectionsInsert {
            id: Uuid::new_v4().into(),
            agent_id: agent_id.clone(),
            ip_address: Some("0.0.0.0".into()),
        },
    )
    .await
    .unwrap();

    agent_id
}
