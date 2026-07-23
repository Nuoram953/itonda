use async_trait::async_trait;

use crate::sync::{context::SyncContext, errors::SyncError};

pub struct MediaSyncPipeline {
    steps: Vec<Box<dyn SyncStep>>,
}

#[async_trait]
pub trait SyncStep: Send + Sync {
    async fn execute(&self, context: &mut SyncContext) -> Result<(), SyncError>;

    fn name(&self) -> &'static str;
}

impl MediaSyncPipeline {
    pub async fn execute(&self, context: &mut SyncContext) -> Result<(), SyncError> {
        for step in &self.steps {
            step.execute(context).await?;
        }

        Ok(())
    }
}
