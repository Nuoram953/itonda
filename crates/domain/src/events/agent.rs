use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize, Clone, PartialEq)]
pub enum AgentEvent {
    Connected { agent_id: Uuid },

    Disconnected { agent_id: Uuid },

    ScanStarted { agent_id: Uuid },

    ScanCompleted { agent_id: Uuid },
}
