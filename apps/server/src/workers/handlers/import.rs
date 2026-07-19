use crate::{
    events::{EventBus, ImportEvent, ImportStep},
    workers::jobs::ImportJob,
};

use itonda_domain::media::import::{MediaImport, import};
use sqlx::SqlitePool;

pub struct ImportHandler {
    db: SqlitePool,
    events: EventBus,
}

impl ImportHandler {
    pub fn new(db: SqlitePool, events: EventBus) -> Self {
        Self { db, events }
    }

    pub async fn handle(&self, job: ImportJob) {
        self.events.publish(ImportEvent::Started { job_id: job.id });

        let total = job.items.len();

        for (index, item) in job.items.into_iter().enumerate() {
            let _ = import(
                &self.db,
                MediaImport {
                    title: item.title.clone(),
                    media_type: item.media_type,
                },
            )
            .await;

            self.events.publish(ImportEvent::Progress {
                job_id: job.id,
                message: format!("Importing {} ({}/{})", item.title.clone(), index + 1, total),
                progress: ((index + 1) * 100 / total) as u8,
                step: ImportStep::Scanning,
            });
        }

        self.events
            .publish(ImportEvent::Completed { job_id: job.id });
    }
}
