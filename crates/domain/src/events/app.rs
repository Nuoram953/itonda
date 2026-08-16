use serde::Serialize;

use super::{AgentEvent, JobEvent, MediaEvent};

#[derive(Debug, Serialize, Clone, PartialEq)]
pub enum AppEvent {
    Job(JobEvent),

    Agent(AgentEvent),

    Media(MediaEvent),
}
