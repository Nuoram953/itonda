use axum::Router;
use sqlx::SqlitePool;
use tokio::sync::mpsc::Sender;
use utoipa_swagger_ui::SwaggerUi;

use itonda_server::{
    api,
    config::{app::AppConfigManager, settings::SettingsManager},
    events::EventBus,
    state::{self, AppState},
    storage::path::AppPaths,
    websocket::{self, AgentManager},
    workers::{handlers::import::ImportHandler, jobs::Job, worker::Worker},
};

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use api::openapi::ApiDoc;
use itonda_database::connection;
use utoipa::OpenApi;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_logging();

    tracing::info!("Starting Itonda server");

    let pool = init_db().await?;

    let (settings, config) = init_config().await?;

    let (jobs, events) = init_worker(&pool).await?;

    let state = state::AppState {
        db: pool,
        jobs,
        events,
        settings,
        config,
        agent_manager: AgentManager::new(),
    };

    init_server(state).await?;

    Ok(())
}

fn init_logging() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "itonda=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Logging initialized");
}

async fn init_config() -> anyhow::Result<(SettingsManager, AppConfigManager)> {
    let paths = AppPaths::new();

    let settings = SettingsManager::load(paths.config_dir.join("settings.toml"))?;

    let config = AppConfigManager::load(paths.config_dir.join("config.toml"))?;

    tracing::info!("Config initialized");

    Ok((settings, config))
}

async fn init_db() -> anyhow::Result<SqlitePool> {
    let pool = connection::connect("sqlite://itonda.db").await;

    connection::migrate(&pool).await?;

    tracing::info!("Database initialized");

    Ok(pool)
}

async fn init_worker(pool: &SqlitePool) -> anyhow::Result<(Sender<Job>, EventBus)> {
    let events = EventBus::new();

    let (sender, receiver) = tokio::sync::mpsc::channel(100);

    let worker = Worker::new(receiver, ImportHandler::new(pool.clone(), events.clone()));

    tokio::spawn(async move {
        worker.run().await;
    });

    tracing::info!("Worker initialized");

    Ok((sender, events))
}

async fn init_server(state: AppState) -> anyhow::Result<()> {
    let url = format!(
        "{}:{}",
        state.config.get().await.server.host,
        state.config.get().await.server.port
    );

    let app = Router::new()
        .nest("/api/v1", api::router())
        .nest("/ws", websocket::router())
        .with_state(state)
        .merge(SwaggerUi::new("/swagger").url("/api-docs/openapi.json", ApiDoc::openapi()));

    let listener = tokio::net::TcpListener::bind(&url).await?;

    tracing::info!("Server running on {}", &url);

    axum::serve(listener, app).await?;

    Ok(())
}
