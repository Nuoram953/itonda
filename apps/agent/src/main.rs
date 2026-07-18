use itonda_domain::protocol::{
    agent::AgentRegistration, message::AgentMessage, server::ServerMessage,
};

use crate::{config::AgentConfigStore, connection::AgentConnection};
use local_ip_address::local_ip;

pub mod config;
pub mod connection;
pub mod identity;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let path = dirs::config_dir().unwrap().join("Itonda");
    let agent_config_store = AgentConfigStore::load(path.join("agent.toml"))?;

    let config = agent_config_store.get().await;

    let registration = AgentRegistration {
        id: config.identity.id.clone(),
        name: config.identity.name.clone(),
        hostname: config.identity.name.clone(),
        platform: std::env::consts::OS.into(),
        agent_version: env!("CARGO_PKG_VERSION").into(),
        ip_address: local_ip().unwrap().to_string(),
    };

    let connection =
        AgentConnection::connect("ws://localhost:3005/ws/agent/connect".into()).await?;

    let agent = Agent::new(connection);

    agent.run(registration).await?;

    Ok(())
}
pub struct Agent {
    connection: AgentConnection,
}

impl Agent {
    pub fn new(connection: AgentConnection) -> Self {
        Self { connection }
    }

    pub async fn run(mut self, registration: AgentRegistration) -> anyhow::Result<()> {
        self.connection
            .send(&AgentMessage::Register(registration))
            .await?;

        loop {
            let command = self.connection.receive().await?;

            match command {
                ServerMessage::Ping => {
                    println!("test ping server -> agent");
                }
            }
        }
    }
}
