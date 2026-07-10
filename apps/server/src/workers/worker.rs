use tokio::sync::mpsc;

use super::{handlers::import::ImportHandler, jobs::Job};

pub struct Worker {
    receiver: mpsc::Receiver<Job>,

    import_handler: ImportHandler,
}

impl Worker {
    pub fn new(receiver: mpsc::Receiver<Job>, import_handler: ImportHandler) -> Self {
        Self {
            receiver,
            import_handler,
        }
    }

    pub async fn run(mut self) {
        while let Some(job) = self.receiver.recv().await {
            match job {
                Job::Import(job) => {
                    self.import_handler.handle(job).await;
                }
            }
        }
    }
}
