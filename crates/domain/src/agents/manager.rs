use std::{collections::HashMap, sync::Arc};
use tokio::sync::{RwLock, mpsc};
use uuid::Uuid;

use crate::{
    agents::errors::AgentsError,
    protocol::{ScanCommand, ServerToAgentMessage},
};

#[derive(Debug, Clone, Default)]
pub struct AgentManager {
    agents: Arc<RwLock<HashMap<String, mpsc::Sender<ServerToAgentMessage>>>>,
}

impl AgentManager {
    pub fn new() -> Self {
        Self {
            agents: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn register(&self, agent_id: String, sender: mpsc::Sender<ServerToAgentMessage>) {
        self.agents.write().await.insert(agent_id, sender);
    }

    pub async fn unregister(&self, agent_id: &str) {
        self.agents.write().await.remove(agent_id);
    }

    pub async fn send(
        &self,
        agent_id: &str,
        command: ServerToAgentMessage,
    ) -> Result<(), AgentsError> {
        let agents = self.agents.read().await;

        let sender = agents
            .get(agent_id)
            .ok_or_else(|| AgentsError::NotConnected(agent_id.to_string()))?;

        sender
            .send(command)
            .await
            .map_err(|e| AgentsError::SendFailed(e.to_string()))?;

        Ok(())
    }

    pub async fn broadcast(&self, command: ServerToAgentMessage) -> Result<(), AgentsError> {
        let agents = self.agents.read().await;
        for sender in agents.values() {
            let _ = sender.send(command.clone()).await;
        }
        Ok(())
    }

    pub async fn scan_all(&self) -> Result<(), AgentsError> {
        self.broadcast(ServerToAgentMessage::Scan(ScanCommand {
            request_id: Uuid::new_v4(),
            media_type: None,
            source: None,
        }))
        .await
    }

    pub async fn get_connected_agent_ids(&self) -> Vec<String> {
        self.agents.read().await.keys().cloned().collect()
    }
}
