use itonda_domain::{
    protocol::{AgentToServerMessage, LaunchCommand, MediaStartedPayload, MediaStoppedPayload},
    tracker::{DirectoryTracker, MediaTracker, TrackTarget},
};
use std::time::Duration;
use tokio::sync::mpsc;
use tracing::{debug, error, info, warn};

pub async fn spawn_media_tracker(
    tx: mpsc::Sender<AgentToServerMessage>,
    agent_id: String,
    command: LaunchCommand,
) {
    let tracker = DirectoryTracker::new();
    track_media_launch(&tracker, tx, agent_id, command).await;
}

pub async fn track_media_launch<T: MediaTracker>(
    tracker: &T,
    tx: mpsc::Sender<AgentToServerMessage>,
    agent_id: String,
    command: LaunchCommand,
) {
    let Some(working_dir) = &command.working_directory else {
        warn!(
            "Cannot track launch {}: no working directory provided",
            command.launch_id
        );
        return;
    };

    info!(
        "Starting media tracker for launch '{}' (media_id='{}', working_dir='{}')",
        command.launch_id, command.media_id, working_dir
    );

    let target = TrackTarget::from_directory(&command.launch_id, working_dir);

    let processes = tracker
        .wait_for_startup(&target, Duration::from_secs(30), Duration::from_secs(1))
        .await
        .unwrap_or_default();

    if processes.is_empty() {
        warn!("Launch {} did not start within timeout", command.launch_id);
        return;
    }

    info!(
        "Launch '{}' process started ({} process(es) detected)",
        command.launch_id,
        processes.len()
    );

    let started_at = chrono::Utc::now();

    if let Err(err) = tx
        .send(AgentToServerMessage::MediaStarted(MediaStartedPayload {
            media_id: command.media_id.clone(),
            agent_id: agent_id.clone(),
            launch_id: command.launch_id.clone(),
            started_at,
        }))
        .await
    {
        error!(
            "Failed to send MediaStarted message for launch '{}': {err}",
            command.launch_id
        );
    } else {
        debug!(
            "Sent MediaStarted payload for launch '{}'",
            command.launch_id
        );
    }

    info!(
        "Waiting for launch '{}' process(es) to exit...",
        command.launch_id
    );
    let _ = tracker
        .wait_until_stopped(&target, Duration::from_secs(2))
        .await;

    let stopped_at = chrono::Utc::now();
    let duration_seconds = (stopped_at - started_at).num_seconds().max(0) as u64;

    info!(
        "Launch '{}' process(es) stopped. Session duration: {}s",
        command.launch_id, duration_seconds
    );

    if let Err(err) = tx
        .send(AgentToServerMessage::MediaStopped(MediaStoppedPayload {
            media_id: command.media_id.clone(),
            agent_id,
            launch_id: command.launch_id.clone(),
            started_at,
            stopped_at,
            duration_seconds,
        }))
        .await
    {
        error!(
            "Failed to send MediaStopped message for launch '{}': {err}",
            command.launch_id
        );
    } else {
        debug!(
            "Sent MediaStopped payload for launch '{}'",
            command.launch_id
        );
    }
}
