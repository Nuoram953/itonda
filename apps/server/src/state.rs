use crate::{
    config::{app::AppConfigManager, secrets::SecretsManager, settings::SettingsManager},
    events::EventBus,
    websocket::AgentManager,
    workers::jobs::Job,
};
use itonda_domain::storefronts::registry::StorefrontRegistry;
use sqlx::SqlitePool;
use tokio::sync::mpsc;

#[derive(Clone)]
pub struct AppState {
    pub db: SqlitePool,
    pub jobs: mpsc::Sender<Job>,
    pub events: EventBus,
    pub settings: SettingsManager,
    pub config: AppConfigManager,
    pub secrets: SecretsManager,
    pub storefronts: StorefrontRegistry,
    pub agent_manager: AgentManager,
}
