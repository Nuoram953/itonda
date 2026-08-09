use async_trait::async_trait;
use tracing::debug;

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
            debug!("Running step {}", step.name());
            step.execute(context).await?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    use crate::{
        sync::{
            context::SyncContext,
            errors::SyncError,
            pipeline::{MediaSyncPipeline, SyncStep},
        },
        tests::fixtures::context::sync_context,
    };

    struct TestStep {
        name: &'static str,
        executed: Arc<Mutex<Vec<String>>>,
    }

    impl TestStep {
        fn new(name: &'static str, executed: Arc<Mutex<Vec<String>>>) -> Self {
            Self { name, executed }
        }
    }

    #[async_trait]
    impl SyncStep for TestStep {
        async fn execute(&self, _context: &mut SyncContext) -> Result<(), SyncError> {
            self.executed.lock().unwrap().push(self.name.to_string());

            Ok(())
        }

        fn name(&self) -> &'static str {
            self.name
        }
    }

    struct FailingStep;

    #[async_trait]
    impl SyncStep for FailingStep {
        fn name(&self) -> &'static str {
            "Persist"
        }
        async fn execute(&self, _: &mut SyncContext) -> Result<(), SyncError> {
            Err(SyncError::MissingMedia)
        }
    }

    struct SetTitleStep;

    #[async_trait]
    impl SyncStep for SetTitleStep {
        fn name(&self) -> &'static str {
            "Persist"
        }
        async fn execute(&self, context: &mut SyncContext) -> Result<(), SyncError> {
            if let Some(discovered) = &mut context.discovered {
                discovered.title = "Portal 2".into();
            }
            Ok(())
        }
    }

    struct AssertTitleStep;

    #[async_trait]
    impl SyncStep for AssertTitleStep {
        fn name(&self) -> &'static str {
            "Persist"
        }

        async fn execute(&self, context: &mut SyncContext) -> Result<(), SyncError> {
            assert_eq!(context.discovered.as_ref().unwrap().title, "Portal 2");
            Ok(())
        }
    }

    #[tokio::test]
    async fn executes_steps_in_order() {
        let executed = Arc::new(Mutex::new(Vec::new()));

        let pipeline = MediaSyncPipeline::new(vec![
            Box::new(TestStep::new("identify", executed.clone())),
            Box::new(TestStep::new("persist", executed.clone())),
        ]);

        let mut context = sync_context();

        pipeline.execute(&mut context).await.unwrap();

        assert_eq!(*executed.lock().unwrap(), vec!["identify", "persist"]);
    }

    #[tokio::test]
    async fn passes_context_between_steps() {
        let pipeline =
            MediaSyncPipeline::new(vec![Box::new(SetTitleStep), Box::new(AssertTitleStep)]);

        let mut context = sync_context();

        pipeline.execute(&mut context).await.unwrap();
    }

    #[tokio::test]
    async fn stops_when_step_fails() {
        let executed = Arc::new(Mutex::new(Vec::new()));

        let pipeline = MediaSyncPipeline::new(vec![
            Box::new(TestStep::new("identify", executed.clone())),
            Box::new(FailingStep),
            Box::new(TestStep::new("persist", executed.clone())),
        ]);

        let mut context = sync_context();

        let result = pipeline.execute(&mut context).await;

        assert!(result.is_err());

        assert_eq!(*executed.lock().unwrap(), vec!["identify"]);
    }

    #[tokio::test]
    async fn executes_empty_pipeline() {
        let pipeline = MediaSyncPipeline::new(vec![]);

        let mut context = sync_context();

        let result = pipeline.execute(&mut context).await;

        assert!(result.is_ok());
    }
}
