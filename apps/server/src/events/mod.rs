use tokio::sync::broadcast;
use tracing::{debug, warn};

mod import;

pub use import::{ImportEvent, ImportStep};

#[derive(Clone)]
pub struct EventBus {
    sender: broadcast::Sender<ImportEvent>,
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}

impl EventBus {
    pub fn new() -> Self {
        let (sender, _) = broadcast::channel(100);

        Self { sender }
    }

    pub fn publish(&self, event: ImportEvent) {
        if self.sender.receiver_count() == 0 {
            debug!("No subscribers, dropping event");
            return;
        }
        match self.sender.send(event.clone()) {
            Ok(receivers) => {
                debug!(?event, receivers, "Event published");
            }

            Err(err) => {
                warn!(?err, ?event, "Failed to publish event");
            }
        }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<ImportEvent> {
        debug!("New event subscriber");

        self.sender.subscribe()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_subscriber() {
        let bus = EventBus::new();

        let _receiver = bus.subscribe();

        assert_eq!(bus.sender.receiver_count(), 1);
    }

    #[tokio::test]
    async fn publishes_import_events() {
        let bus = EventBus::new();

        let mut receiver = bus.subscribe();

        let event = ImportEvent::Started {
            job_id: uuid::Uuid::new_v4(),
        };

        bus.publish(event.clone());

        let received = receiver.recv().await.unwrap();

        assert_eq!(received, event);
    }

    #[tokio::test]
    async fn publishes_event_to_multiple_subscribers() {
        let bus = EventBus::new();

        let mut receiver_a = bus.subscribe();
        let mut receiver_b = bus.subscribe();

        let event = ImportEvent::Started {
            job_id: uuid::Uuid::new_v4(),
        };

        bus.publish(event.clone());

        assert_eq!(receiver_a.recv().await.unwrap(), event);
        assert_eq!(receiver_b.recv().await.unwrap(), event);
    }

    #[test]
    fn publishing_without_subscribers_does_not_panic() {
        let bus = EventBus::new();

        let event = ImportEvent::Started {
            job_id: uuid::Uuid::new_v4(),
        };

        bus.publish(event);
    }

    #[tokio::test]
    async fn new_subscriber_does_not_receive_previous_events() {
        let bus = EventBus::new();

        let event = ImportEvent::Started {
            job_id: uuid::Uuid::new_v4(),
        };

        bus.publish(event);

        let mut receiver = bus.subscribe();

        assert!(receiver.try_recv().is_err());
    }

    #[tokio::test]
    async fn dropped_subscriber_does_not_break_publishing() {
        let bus = EventBus::new();

        {
            let _receiver = bus.subscribe();
        }

        let event = ImportEvent::Started {
            job_id: uuid::Uuid::new_v4(),
        };

        bus.publish(event);
    }
}
