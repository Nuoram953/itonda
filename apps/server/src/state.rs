use crate::{
    config::{app::AppConfigManager, settings::SettingsManager},
    events::EventBus,
    websocket::AgentManager,
    workers::jobs::Job,
};
use sqlx::SqlitePool;
use tokio::sync::mpsc;

#[derive(Clone)]
pub struct AppState {
    pub db: SqlitePool,
    pub jobs: mpsc::Sender<Job>,
    pub events: EventBus,
    pub settings: SettingsManager,
    pub config: AppConfigManager,
    pub agent_manager: AgentManager,
}
