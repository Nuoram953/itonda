use axum::Router;
use itonda_database::test_utils::setup_db;
use itonda_server::{
    api,
    config::{app::AppConfigManager, settings::SettingsManager},
    events::EventBus,
    state::AppState,
    websocket::AgentManager,
    workers::jobs::Job,
};
use sqlx::SqlitePool;
use tempfile::tempdir;
use tokio::sync::mpsc::Receiver;

pub struct TestApp {
    pub router: Router,
    pub jobs: Receiver<Job>,
    pub db: SqlitePool,
}

pub async fn test_app() -> TestApp {
    let db = setup_db().await;

    let temp = tempdir().unwrap();

    let settings = SettingsManager::load(temp.path().join("settings.toml")).unwrap();

    let config = AppConfigManager::load(temp.path().join("config.toml")).unwrap();

    let (jobs, receiver) = tokio::sync::mpsc::channel(100);

    let state = AppState {
        db: db.clone(),
        jobs,
        events: EventBus::new(),
        settings,
        config,
        agent_manager: AgentManager::new(),
    };

    TestApp {
        router: api::router().with_state(state),
        jobs: receiver,
        db,
    }
}
