use async_trait::async_trait;

use crate::sync::{context::SyncContext, errors::SyncError, pipeline::SyncStep};

pub struct IdentifyStep {}

impl IdentifyStep {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl SyncStep for IdentifyStep {
    fn name(&self) -> &'static str {
        "Identify"
    }

    async fn execute(&self, context: &mut SyncContext) -> Result<(), SyncError> {
        Ok(())
    }
}
