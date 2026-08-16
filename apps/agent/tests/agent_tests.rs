use async_trait::async_trait;
use std::time::Duration;
use tokio::sync::mpsc;
use uuid::Uuid;

use itonda_agent::{
    AgentConfig, AgentIdentity, config::DEFAULT_SERVER_URL, tracker::track_media_launch,
};
use itonda_domain::{
    protocol::{AgentToServerMessage, LaunchCommand},
    tracker::{MediaTracker, ProcessInfo, TrackTarget, errors::TrackerError},
};

#[test]
fn test_agent_config_defaults() {
    let config = AgentConfig::default();
    assert_eq!(config.server.url, DEFAULT_SERVER_URL);
    assert_eq!(config.identity.name, "Itonda Agent");
    assert!(Uuid::parse_str(&config.identity.id).is_ok());
}

#[test]
fn test_agent_config_toml_roundtrip() {
    let toml_str = r#"
        [identity]
        id = "test-agent-123"
        name = "Custom Agent"

        [server]
        url = "ws://192.168.1.50:3005/ws/agent/connect"
    "#;

    let config: AgentConfig = toml::from_str(toml_str).unwrap();
    assert_eq!(config.identity.id, "test-agent-123");
    assert_eq!(config.identity.name, "Custom Agent");
    assert_eq!(config.server.url, "ws://192.168.1.50:3005/ws/agent/connect");
}

#[test]
fn test_agent_config_toml_without_server_section() {
    let toml_str = r#"
        [identity]
        id = "legacy-agent-456"
        name = "Legacy Agent"
    "#;

    let config: AgentConfig = toml::from_str(toml_str).unwrap();
    assert_eq!(config.identity.id, "legacy-agent-456");
    assert_eq!(config.identity.name, "Legacy Agent");
    assert_eq!(config.server.url, DEFAULT_SERVER_URL);
}

#[test]
fn test_agent_identity_constructors() {
    let id1 = AgentIdentity::new("Agent Alpha");
    assert_eq!(id1.name, "Agent Alpha");
    assert!(Uuid::parse_str(&id1.id).is_ok());

    let id2 = AgentIdentity::with_id("custom-uuid", "Agent Beta");
    assert_eq!(id2.id, "custom-uuid");
    assert_eq!(id2.name, "Agent Beta");
}

struct MockTracker {
    processes: Vec<ProcessInfo>,
}

#[async_trait]
impl MediaTracker for MockTracker {
    fn name(&self) -> &'static str {
        "mock-tracker"
    }

    async fn find_processes(
        &self,
        _target: &TrackTarget,
    ) -> Result<Vec<ProcessInfo>, TrackerError> {
        Ok(self.processes.clone())
    }

    async fn wait_for_startup(
        &self,
        _target: &TrackTarget,
        _buffer_timeout: Duration,
        _poll_interval: Duration,
    ) -> Result<Vec<ProcessInfo>, TrackerError> {
        Ok(self.processes.clone())
    }

    async fn wait_until_stopped(
        &self,
        _target: &TrackTarget,
        _poll_interval: Duration,
    ) -> Result<(), TrackerError> {
        Ok(())
    }
}

#[tokio::test]
async fn test_track_media_launch_successful_lifecycle() {
    let tracker = MockTracker {
        processes: vec![ProcessInfo {
            pid: 1234,
            name: "test_game.exe".into(),
            exe: None,
            cwd: None,
        }],
    };

    let (tx, mut rx) = mpsc::channel(10);
    let agent_id = "agent-test-1".to_string();
    let command = LaunchCommand {
        request_id: Uuid::new_v4(),
        media_id: "media-42".into(),
        launch_id: "launch-99".into(),
        program: "/games/test_game.exe".into(),
        args: vec![],
        working_directory: Some("/games/test_game".into()),
    };

    track_media_launch(&tracker, tx, agent_id.clone(), command).await;

    let started_msg = rx.recv().await.expect("Expected MediaStarted message");
    match started_msg {
        AgentToServerMessage::MediaStarted(payload) => {
            assert_eq!(payload.media_id, "media-42");
            assert_eq!(payload.launch_id, "launch-99");
            assert_eq!(payload.agent_id, agent_id);
        }
        other => panic!("Expected MediaStarted, got {:?}", other),
    }

    let stopped_msg = rx.recv().await.expect("Expected MediaStopped message");
    match stopped_msg {
        AgentToServerMessage::MediaStopped(payload) => {
            assert_eq!(payload.media_id, "media-42");
            assert_eq!(payload.launch_id, "launch-99");
            assert_eq!(payload.agent_id, agent_id);
        }
        other => panic!("Expected MediaStopped, got {:?}", other),
    }
}

#[tokio::test]
async fn test_track_media_launch_missing_working_directory() {
    let tracker = MockTracker { processes: vec![] };

    let (tx, mut rx) = mpsc::channel(10);
    let agent_id = "agent-test-1".to_string();
    let command = LaunchCommand {
        request_id: Uuid::new_v4(),
        media_id: "media-42".into(),
        launch_id: "launch-99".into(),
        program: "/games/test_game.exe".into(),
        args: vec![],
        working_directory: None,
    };

    track_media_launch(&tracker, tx, agent_id, command).await;

    assert!(rx.try_recv().is_err());
}
