use std::{collections::HashMap, sync::Arc};

use axum::{
    extract::{
        State,
        ws::{Message, WebSocket, WebSocketUpgrade},
    },
    response::Response,
};
use itonda_database::agent::{
    AgentConnectionsInsert, AgentsInsert, disconnect_agent_connection, insert_agent_connection,
    upsert_agent,
};
use itonda_domain::protocol::{agent::AgentRegistration, message::AgentMessage};
use sqlx::SqlitePool;
use tokio::sync::{RwLock, mpsc};
use tracing::{debug, warn};
use uuid::Uuid;

use crate::state::AppState;

#[derive(Debug, Clone)]
pub struct AgentManager {
    agents: Arc<RwLock<HashMap<String, mpsc::Sender<AgentMessage>>>>,
}

impl Default for AgentManager {
    fn default() -> Self {
        Self::new()
    }
}

impl AgentManager {
    pub fn new() -> Self {
        Self {
            agents: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    pub async fn register(&self, agent_id: String, sender: mpsc::Sender<AgentMessage>) {
        self.agents.write().await.insert(agent_id, sender);
    }

    pub async fn send(&self, agent_id: &str, command: AgentMessage) -> anyhow::Result<()> {
        let agents = self.agents.read().await;

        let sender = agents
            .get(agent_id)
            .ok_or_else(|| anyhow::anyhow!("Agent not connected"))?;

        sender.send(command).await?;

        Ok(())
    }
}

pub async fn agent_ws(ws: WebSocketUpgrade, State(state): State<AppState>) -> Response {
    let manager = state.agent_manager.clone();

    ws.on_upgrade(move |socket| async move {
        if let Err(err) = handle_agent(socket, &state.db, manager).await {
            tracing::error!("Agent connection error: {err}");
        }
    })
}

async fn handle_agent(
    mut socket: WebSocket,
    pool: &SqlitePool,
    agent_manager: AgentManager,
) -> anyhow::Result<()> {
    debug!("Waiting for registration");

    let registration = wait_for_registration(&mut socket).await?;

    debug!("Registered agent: {:?}", registration);

    let agent_id = registration.id.clone();

    upsert_agent(
        pool,
        AgentsInsert {
            id: registration.id,
            name: registration.name,
            hostname: registration.hostname,
            platform: registration.platform,
            agent_version: registration.agent_version,
        },
    )
    .await?;

    insert_agent_connection(
        pool,
        AgentConnectionsInsert {
            id: Uuid::new_v4().into(),
            agent_id: agent_id.clone(),
            ip_address: Some(registration.ip_address),
        },
    )
    .await?;

    let (tx, mut rx) = mpsc::channel(32);

    agent_manager.register(agent_id.clone(), tx).await;

    loop {
        tokio::select! {
            Some(command) = rx.recv() => {
                debug!("Sending command: {:?}", command);

                let message = serde_json::to_string(&command).unwrap();

                socket
                    .send(axum::extract::ws::Message::Text(message.into()))
                    .await
                    .unwrap();
            }

            Some(message) = socket.recv() => {
                debug!("Received from agent: {:?}", message);
                match message{
                    Ok(_) => {
                    }

                    Err(err) => {
                        warn!("Error message: {:?}", err);
                        disconnect_agent_connection(pool, agent_id.clone()).await?;
                    }
                }
            }

        }
    }
}

async fn wait_for_registration(socket: &mut WebSocket) -> anyhow::Result<AgentRegistration> {
    while let Some(message) = socket.recv().await {
        match message? {
            Message::Text(text) => {
                debug!("Raw agent message: {}", text);
                let message: AgentMessage = serde_json::from_str(&text)?;

                debug!("Parsed: {:?}", message);

                match message {
                    AgentMessage::Register(registration) => {
                        return Ok(registration);
                    }

                    AgentMessage::Ping => {}
                }
            }

            Message::Close(_) => {
                anyhow::bail!("Agent disconnected before registering");
            }

            _ => {}
        }
    }

    anyhow::bail!("Socket closed");
}
