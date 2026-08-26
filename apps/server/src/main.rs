use std::{fs::File, sync::Arc};

use axum::Router;
use itonda_domain::{
    assets::{
        registry::AssetRegistry, steam_grid_db::SteamGridDb, the_movie_database::TheMovieDatabase,
    },
    events::EventBus,
    metadata::{registry::MetadataRegistry, the_internet_game_database::TheInternetGameDatabase},
    storage::path::AppPaths,
    store::toml::TomlCodec,
    storefronts::{registry::StorefrontRegistry, steam::SteamStorefront},
};
use sqlx::SqlitePool;
use tokio::sync::mpsc::Sender;
use tracing::level_filters::LevelFilter;
use utoipa_swagger_ui::SwaggerUi;

use itonda_server::{
    api,
    config::{app::AppConfigManager, secrets::SecretsManager, settings::SettingsManager},
    state::{self, AppState},
    websocket::{self, AgentManager},
    workers::{
        handlers::{import::ImportHandler, sync::SyncHandler},
        jobs::Job,
        worker::Worker,
    },
};

use tracing_subscriber::{layer::SubscriberExt, prelude::*, util::SubscriberInitExt};

use api::openapi::ApiDoc;
use itonda_database::connection;
use utoipa::OpenApi;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_logging();

    tracing::info!("Starting Itonda server");

    let pool = init_db().await?;

    let (settings, config, secrets) = init_config().await?;

    let storefronts = init_storefronts(&secrets).await?;

    let asset_store = init_asset_store(&secrets).await?;

    let metadata = init_metadata(&secrets).await?;

    let agent_manager = AgentManager::new();

    let (jobs, events) =
        init_worker(&pool, &storefronts, &asset_store, &metadata, &agent_manager).await?;

    let state = state::AppState {
        db: pool,
        jobs,
        events,
        settings,
        config,
        secrets,
        storefronts,
        metadata,
        agent_manager,
    };

    init_server(state).await?;

    Ok(())
}

fn init_logging() {
    let paths = AppPaths::new();
    let log_file =
        File::create(paths.log_dir().join("debug.txt")).expect("Failed to create log file");

    let stdout_layer = tracing_subscriber::fmt::layer()
        .with_writer(std::io::stdout)
        .with_filter(LevelFilter::INFO);

    let file_layer = tracing_subscriber::fmt::layer()
        .with_writer(log_file)
        .with_ansi(false)
        .with_filter(LevelFilter::DEBUG);

    tracing_subscriber::registry()
        .with(stdout_layer)
        .with(file_layer)
        .init();

    tracing::info!("Logging initialized");
}

async fn init_config() -> anyhow::Result<(SettingsManager, AppConfigManager, SecretsManager)> {
    let paths = AppPaths::new();

    let settings = SettingsManager::load(paths.config_dir.join("settings.toml"), TomlCodec)?;

    let config = AppConfigManager::load(paths.config_dir.join("config.toml"), TomlCodec)?;

    let secrets = SecretsManager::load(paths.config_dir.join("secrets.toml"), TomlCodec)?;

    tracing::info!("Config initialized");

    Ok((settings, config, secrets))
}

async fn init_db() -> anyhow::Result<SqlitePool> {
    let pool = connection::connect("sqlite://itonda.db").await;

    if std::env::var("RUN_MIGRATIONS").is_ok() {
        connection::migrate(&pool).await?;
    }

    let _ = itonda_database::agent::disconnect_all_agent_connections(&pool).await;

    tracing::info!("Database initialized");

    Ok(pool)
}

async fn init_worker(
    pool: &SqlitePool,
    storefronts: &StorefrontRegistry,
    asset_store: &AssetRegistry,
    metadata: &MetadataRegistry,
    agent_manager: &AgentManager,
) -> anyhow::Result<(Sender<Job>, EventBus)> {
    let events = EventBus::new();

    let (sender, receiver) = tokio::sync::mpsc::channel(100);

    let worker = Worker::new(
        receiver,
        ImportHandler::new(pool.clone(), events.clone()),
        SyncHandler::new(
            pool.clone(),
            events.clone(),
            agent_manager.clone(),
            storefronts.clone(),
            asset_store.clone(),
            metadata.clone(),
        ),
    );

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

async fn init_storefronts(secrets: &SecretsManager) -> anyhow::Result<StorefrontRegistry> {
    let secrets = secrets.get().await;

    let registry = StorefrontRegistry::new();

    if !secrets.storefronts.steam.steam_id.is_empty() && secrets.storefronts.steam.steam_id != "0" {
        registry.register(Arc::new(SteamStorefront::new(
            secrets.storefronts.steam.api_key,
            secrets.storefronts.steam.steam_id,
        )));
    }

    Ok(registry)
}

async fn init_asset_store(secrets: &SecretsManager) -> anyhow::Result<AssetRegistry> {
    let secrets = secrets.get().await;

    let mut registry = AssetRegistry::new();

    registry.register_poster(Arc::new(SteamGridDb::new(
        secrets.asset_store.steam_grid_db.api_key.clone(),
    )));

    registry.register_banner(Arc::new(SteamGridDb::new(
        secrets.asset_store.steam_grid_db.api_key.clone(),
    )));

    registry.register_poster(Arc::new(TheMovieDatabase::new(
        secrets.asset_store.tmdb.api_key,
    )));

    Ok(registry)
}

async fn init_metadata(secrets: &SecretsManager) -> anyhow::Result<MetadataRegistry> {
    let secrets = secrets.get().await;

    let mut registry = MetadataRegistry::new();

    if !secrets.metadata_store.igdb.client_id.is_empty()
        && !secrets.metadata_store.igdb.client_secret.is_empty()
    {
        registry.register(Arc::new(TheInternetGameDatabase::new(
            secrets.metadata_store.igdb.client_id,
            secrets.metadata_store.igdb.client_secret,
        )));
    }

    Ok(registry)
}
