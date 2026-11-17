use std::{collections::HashMap, sync::Arc, time::Duration};
use tokio::sync::{RwLock, mpsc, oneshot};
use uuid::Uuid;

use crate::{
    agents::errors::AgentsError,
    protocol::{DirectoryListingResult, ListDirectoryCommand, ScanCommand, ServerToAgentMessage},
};

#[derive(Debug, Clone, Default)]
pub struct AgentManager {
    agents: Arc<RwLock<HashMap<String, mpsc::Sender<ServerToAgentMessage>>>>,
    pending_fs: Arc<RwLock<HashMap<Uuid, oneshot::Sender<DirectoryListingResult>>>>,
}

impl AgentManager {
    pub fn new() -> Self {
        Self {
            agents: Arc::new(RwLock::new(HashMap::new())),
            pending_fs: Arc::new(RwLock::new(HashMap::new())),
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

    pub async fn list_directory(
        &self,
        agent_id: Option<&str>,
        path: Option<String>,
    ) -> Result<DirectoryListingResult, AgentsError> {
        let (target_agent_id, sender) = {
            let agents = self.agents.read().await;
            if let Some(id) = agent_id.map(str::trim).filter(|id| !id.is_empty()) {
                let sender = agents
                    .get(id)
                    .cloned()
                    .ok_or_else(|| AgentsError::NotConnected(id.to_string()))?;
                (id.to_string(), sender)
            } else {
                let (id, sender) = agents
                    .iter()
                    .next()
                    .ok_or_else(|| AgentsError::NotConnected("no connected agents".to_string()))?;
                (id.clone(), sender.clone())
            }
        };

        let request_id = Uuid::new_v4();
        let (tx, rx) = oneshot::channel();
        self.pending_fs.write().await.insert(request_id, tx);

        if let Err(err) = sender
            .send(ServerToAgentMessage::ListDirectory(ListDirectoryCommand {
                request_id,
                path,
            }))
            .await
        {
            self.pending_fs.write().await.remove(&request_id);
            return Err(AgentsError::SendFailed(format!(
                "failed to send directory request to {target_agent_id}: {err}"
            )));
        }

        match tokio::time::timeout(Duration::from_secs(10), rx).await {
            Ok(Ok(result)) => Ok(result),
            Ok(Err(_)) => {
                self.pending_fs.write().await.remove(&request_id);
                Err(AgentsError::SendFailed(
                    "directory listing channel closed".to_string(),
                ))
            }
            Err(_) => {
                self.pending_fs.write().await.remove(&request_id);
                Err(AgentsError::SendFailed(
                    "timed out waiting for agent directory listing".to_string(),
                ))
            }
        }
    }

    pub async fn complete_directory_listing(&self, result: DirectoryListingResult) {
        if let Some(tx) = self.pending_fs.write().await.remove(&result.request_id) {
            let _ = tx.send(result);
        }
    }
}
