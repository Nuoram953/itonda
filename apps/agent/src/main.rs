use std::{fs::File, sync::Arc, time::Duration};

use itonda_agent::{Agent, config::AgentConfigStore, connection::AgentConnection};
use itonda_domain::{
    protocol::AgentRegistration,
    scanner::{registry::ScannerRegistry, steam::SteamScanner},
    storage::path::AgentPaths,
    store::toml::TomlCodec,
};
use local_ip_address::local_ip;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{layer::SubscriberExt, prelude::*, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_logging();

    tracing::info!("Starting Itonda agent");

    let paths = AgentPaths::new();
    let agent_config_store =
        AgentConfigStore::load(paths.config_dir.join("agent.toml"), TomlCodec)?;

    let config = agent_config_store.get().await;

    tracing::info!(
        "Loaded config for agent '{}' ({})",
        config.identity.name,
        config.identity.id
    );

    let ip_address = local_ip()
        .map(|ip| ip.to_string())
        .unwrap_or_else(|_| "127.0.0.1".to_string());

    let hostname = hostname::get()
        .ok()
        .and_then(|h| h.into_string().ok())
        .unwrap_or_else(|| config.identity.name.clone());

    let registration = AgentRegistration {
        id: config.identity.id.clone(),
        name: config.identity.name.clone(),
        hostname,
        platform: std::env::consts::OS.into(),
        agent_version: env!("CARGO_PKG_VERSION").into(),
        ip_address,
    };

    let mut scanner_registry = ScannerRegistry::new();
    scanner_registry.register(Arc::new(SteamScanner::new()));
    let scanner_registry = Arc::new(scanner_registry);

    tracing::info!(
        "Scanner registry initialized with {} scanner(s)",
        scanner_registry.scanners().len()
    );

    let server_url = config.server_url();
    run_agent_loop(server_url, registration, scanner_registry).await;

    Ok(())
}

async fn run_agent_loop(
    server_url: String,
    registration: AgentRegistration,
    scanner_registry: Arc<ScannerRegistry>,
) {
    let mut retry_delay = Duration::from_secs(1);
    let max_delay = Duration::from_secs(30);

    loop {
        tracing::info!("Connecting to server at {}", server_url);

        tokio::select! {
            _ = tokio::signal::ctrl_c() => {
                tracing::info!("Received shutdown signal. Exiting agent.");
                break;
            }
            conn_result = AgentConnection::connect(&server_url) => {
                match conn_result {
                    Ok(connection) => {
                        tracing::info!("Successfully connected to server");
                        retry_delay = Duration::from_secs(1);

                        let agent = Agent::new(connection, scanner_registry.clone());
                        if let Err(err) = agent.run_session(registration.clone()).await {
                            tracing::warn!("Agent session ended with error: {err}");
                        }
                    }
                    Err(err) => {
                        tracing::warn!("Failed to connect to server ({err}). Retrying in {:?}", retry_delay);
                    }
                }
            }
        }

        tokio::select! {
            _ = tokio::signal::ctrl_c() => {
                tracing::info!("Received shutdown signal. Exiting agent.");
                break;
            }
            _ = tokio::time::sleep(retry_delay) => {
                retry_delay = (retry_delay * 2).min(max_delay);
            }
        }
    }
}

fn init_logging() {
    let paths = AgentPaths::new();
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
