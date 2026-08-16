use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum MediaEvent {
    Launched {
        media_id: String,
        launch_id: String,
        agent_id: String,
    },
    Stopped {
        media_id: String,
        launch_id: String,
        agent_id: String,
        duration_seconds: u64,
    },
}
