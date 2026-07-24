use async_trait::async_trait;

use crate::sync::{context::SyncContext, errors::SyncError};

#[async_trait]
pub trait SyncStep: Send + Sync {
    async fn execute(&self, context: &mut SyncContext) -> Result<(), SyncError>;

    fn name(&self) -> &'static str;
}

pub struct MediaSyncPipeline {
    steps: Vec<Box<dyn SyncStep>>,
}

impl MediaSyncPipeline {
    pub fn new(steps: Vec<Box<dyn SyncStep>>) -> Self {
        Self { steps }
    }

    pub async fn execute(&self, context: &mut SyncContext) -> Result<(), SyncError> {
        for step in &self.steps {
            step.execute(context).await?;
        }

        Ok(())
    }
}
