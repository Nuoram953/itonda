use std::sync::Arc;
use uuid::Uuid;

use itonda_domain::{
    launch::service::launch_program_with_command,
    protocol::{AgentRegistration, AgentToServerMessage, ScanResult, ServerToAgentMessage},
    scanner::{registry::ScannerRegistry, steam::SteamScanner},
    store::toml::TomlCodec,
};

use crate::{config::AgentConfigStore, connection::AgentConnection};
use local_ip_address::local_ip;

pub mod config;
pub mod connection;
pub mod identity;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let path = dirs::config_dir().unwrap().join("Itonda");
    let agent_config_store = AgentConfigStore::load(path.join("agent.toml"), TomlCodec)?;

    let config = agent_config_store.get().await;

    let registration = AgentRegistration {
        id: config.identity.id.clone(),
        name: config.identity.name.clone(),
        hostname: config.identity.name.clone(),
        platform: std::env::consts::OS.into(),
        agent_version: env!("CARGO_PKG_VERSION").into(),
        ip_address: local_ip().unwrap().to_string(),
    };

    let mut scanner_registry = ScannerRegistry::new();
    scanner_registry.register(Arc::new(SteamScanner::new()));

    let connection =
        AgentConnection::connect("ws://localhost:3005/ws/agent/connect".into()).await?;

    let agent = Agent::new(connection, scanner_registry);

    agent.run(registration).await?;

    Ok(())
}

pub struct Agent {
    connection: AgentConnection,
    scanner_registry: ScannerRegistry,
}

impl Agent {
    pub fn new(connection: AgentConnection, scanner_registry: ScannerRegistry) -> Self {
        Self {
            connection,
            scanner_registry,
        }
    }

    pub async fn run(mut self, registration: AgentRegistration) -> anyhow::Result<()> {
        let agent_id = registration.id.clone();

        self.connection
            .send(&AgentToServerMessage::Register(registration))
            .await?;

        let scanned_items = self.scanner_registry.scan_all().await;
        let _ = self
            .connection
            .send(&AgentToServerMessage::ScanResult(ScanResult {
                request_id: Uuid::nil(),
                agent_id: agent_id.clone(),
                items: scanned_items,
            }))
            .await;

        loop {
            let command = self.connection.receive().await?;

            match command {
                ServerToAgentMessage::Ping => {
                    let _ = self.connection.send(&AgentToServerMessage::Pong).await;
                }
                ServerToAgentMessage::Launch(command) => {
                    let _ = launch_program_with_command(&command);
                }
                ServerToAgentMessage::Scan(command) => {
                    let items = match command.media_type {
                        Some(mt) => self.scanner_registry.scan_media_type(mt).await,
                        None => self.scanner_registry.scan_all().await,
                    };
                    let _ = self
                        .connection
                        .send(&AgentToServerMessage::ScanResult(ScanResult {
                            request_id: command.request_id,
                            agent_id: agent_id.clone(),
                            items,
                        }))
                        .await;
                }
            }
        }
    }
}
