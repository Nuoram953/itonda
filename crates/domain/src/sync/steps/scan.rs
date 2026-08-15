use async_trait::async_trait;

use crate::{
    agents::AgentManager,
    sync::{context::SyncContext, errors::SyncError, pipeline::SyncStep},
};

pub struct ScanStep {
    agents: AgentManager,
}

impl ScanStep {
    pub fn new(agents: AgentManager) -> Self {
        Self { agents }
    }

    pub async fn scan(&self) -> Result<(), SyncError> {
        let _ = self.agents.scan_all().await;
        Ok(())
    }
}

#[async_trait]
impl SyncStep for ScanStep {
    fn name(&self) -> &'static str {
        "Scan"
    }

    async fn execute(&self, _context: &mut SyncContext) -> Result<(), SyncError> {
        self.scan().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::ServerToAgentMessage;
    use crate::tests::fixtures::context::sync_context;
    use tokio::sync::mpsc;

    #[tokio::test]
    async fn executes_scan_step_successfully() {
        let agents = AgentManager::new();
        let (tx, mut rx) = mpsc::channel(10);
        agents.register("agent-1".into(), tx).await;

        let step = ScanStep::new(agents);
        let mut context = sync_context();

        let result = step.execute(&mut context).await;
        assert!(result.is_ok());

        let message = rx.recv().await;
        assert!(matches!(message, Some(ServerToAgentMessage::Scan(_))));
    }
}
