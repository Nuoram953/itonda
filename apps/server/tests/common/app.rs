use axum::Router;
use itonda_database::test_utils::setup_db;
use itonda_domain::{store::toml::TomlCodec, storefronts::registry::StorefrontRegistry};
use itonda_server::{
    api,
    config::{app::AppConfigManager, secrets::SecretsManager, settings::SettingsManager},
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

    let settings = SettingsManager::load(temp.path().join("settings.toml"), TomlCodec).unwrap();

    let config = AppConfigManager::load(temp.path().join("config.toml"), TomlCodec).unwrap();

    let secrets = SecretsManager::load(temp.path().join("secrets.toml"), TomlCodec).unwrap();

    let (jobs, receiver) = tokio::sync::mpsc::channel(100);

    let state = AppState {
        db: db.clone(),
        jobs,
        events: EventBus::new(),
        settings,
        config,
        secrets,
        storefronts: StorefrontRegistry::new(),
        agent_manager: AgentManager::new(),
    };

    TestApp {
        router: api::router().with_state(state),
        jobs: receiver,
        db,
    }
}
