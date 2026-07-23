use crate::{events::EventBus, workers::jobs::SyncJob};

use sqlx::SqlitePool;

pub struct SyncHandler {
    db: SqlitePool,
    events: EventBus,
}

impl SyncHandler {
    pub fn new(db: SqlitePool, events: EventBus) -> Self {
        Self { db, events }
    }

    pub async fn handle(&self, _job: SyncJob) {}
}
