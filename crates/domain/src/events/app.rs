use serde::Serialize;

use super::{AgentEvent, JobEvent};

#[derive(Debug, Serialize, Clone, PartialEq)]
pub enum AppEvent {
    Job(JobEvent),

    Agent(AgentEvent),
}
