use async_trait::async_trait;
use std::time::Duration;

use crate::tracker::{
    errors::TrackerError,
    models::{ProcessInfo, TrackTarget},
};

pub const DEFAULT_STARTUP_BUFFER: Duration = Duration::from_secs(30);
pub const DEFAULT_POLL_INTERVAL: Duration = Duration::from_secs(5);

#[async_trait]
pub trait MediaTracker: Send + Sync {
    fn name(&self) -> &'static str;

    fn is_available(&self) -> bool {
        true
    }

    async fn find_processes(&self, target: &TrackTarget) -> Result<Vec<ProcessInfo>, TrackerError>;

    async fn is_running(&self, target: &TrackTarget) -> Result<bool, TrackerError> {
        let processes = self.find_processes(target).await?;
        Ok(!processes.is_empty())
    }

    async fn wait_for_startup(
        &self,
        target: &TrackTarget,
        buffer_timeout: Duration,
        poll_interval: Duration,
    ) -> Result<Vec<ProcessInfo>, TrackerError> {
        let start = std::time::Instant::now();
        loop {
            let processes = self.find_processes(target).await?;
            if !processes.is_empty() {
                return Ok(processes);
            }

            if start.elapsed() >= buffer_timeout {
                return Ok(Vec::new());
            }

            tokio::time::sleep(poll_interval).await;
        }
    }

    async fn wait_until_stopped(
        &self,
        target: &TrackTarget,
        poll_interval: Duration,
    ) -> Result<(), TrackerError> {
        let max_missed = 2;
        let mut missed = 0;

        loop {
            tokio::time::sleep(poll_interval).await;

            match self.is_running(target).await {
                Ok(true) => {
                    missed = 0;
                }
                Ok(false) | Err(_) => {
                    missed += 1;
                    if missed >= max_missed {
                        break;
                    }
                }
            }
        }
        Ok(())
    }
}
