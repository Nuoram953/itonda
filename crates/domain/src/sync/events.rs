use uuid::Uuid;

pub enum SyncEvent {
    Started { title: String },

    StepStarted { step: String },

    StepCompleted { step: String },

    Completed { media_id: Uuid },
}
