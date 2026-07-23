use crate::workers::jobs::ImportJob;

use itonda_domain::{
    events::{EventBus, ImportEvent, JobEventType, JobType},
    media::import::{MediaImport, import},
};
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
        self.events.publish_job(
            job.id,
            JobType::Import,
            JobEventType::Import(ImportEvent::Started),
        );

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

            self.events.publish_job(
                job.id,
                JobType::Import,
                JobEventType::Import(ImportEvent::Progress {
                    message: format!("Importing {} ({}/{})", item.title.clone(), index + 1, total),
                    progress: ((index + 1) * 100 / total) as u8,
                }),
            );
        }

        self.events.publish_job(
            job.id,
            JobType::Import,
            JobEventType::Import(ImportEvent::Completed),
        );
    }
}
