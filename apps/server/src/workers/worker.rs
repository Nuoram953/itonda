use tokio::sync::mpsc;

use crate::workers::handlers::sync::SyncHandler;

use super::{handlers::import::ImportHandler, jobs::Job};

pub struct Worker {
    receiver: mpsc::Receiver<Job>,

    import_handler: ImportHandler,
    sync_handler: SyncHandler,
}

impl Worker {
    pub fn new(
        receiver: mpsc::Receiver<Job>,
        import_handler: ImportHandler,
        sync_handler: SyncHandler,
    ) -> Self {
        Self {
            receiver,
            import_handler,
            sync_handler,
        }
    }

    pub async fn run(mut self) {
        while let Some(job) = self.receiver.recv().await {
            match job {
                Job::Import(job) => {
                    self.import_handler.handle(job).await;
                }
                Job::Sync(job) => {
                    self.sync_handler.handle(job).await;
                }
            }
        }
    }
}
