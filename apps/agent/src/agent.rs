use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::{debug, error, info, trace};
use uuid::Uuid;

use itonda_domain::{
    launch::service::launch_program_with_command,
    protocol::{
        AgentRegistration, AgentToServerMessage, LaunchCommand, ScanCommand, ScanResult,
        ServerToAgentMessage,
    },
    scanner::registry::ScannerRegistry,
};

use crate::{connection::AgentConnection, tracker::spawn_media_tracker};

pub struct Agent {
    connection: AgentConnection,
    scanner_registry: Arc<ScannerRegistry>,
    tx: mpsc::Sender<AgentToServerMessage>,
    rx: mpsc::Receiver<AgentToServerMessage>,
}

impl Agent {
    pub fn new(connection: AgentConnection, scanner_registry: Arc<ScannerRegistry>) -> Self {
        let (tx, rx) = mpsc::channel(32);
        Self {
            connection,
            scanner_registry,
            tx,
            rx,
        }
    }

    pub fn sender(&self) -> mpsc::Sender<AgentToServerMessage> {
        self.tx.clone()
    }

    pub async fn run_session(mut self, registration: AgentRegistration) -> anyhow::Result<()> {
        let agent_id = registration.id.clone();

        info!(
            "Registering agent with server: id={}, name={}, version={}",
            registration.id, registration.name, registration.agent_version
        );

        self.connection
            .send(&AgentToServerMessage::Register(registration))
            .await?;
        debug!("Registration dispatched to server");

        info!("Performing initial media scan...");
        let scanned_items = self.scanner_registry.scan_all().await;
        info!(
            "Initial media scan complete: found {} item(s)",
            scanned_items.len()
        );

        self.connection
            .send(&AgentToServerMessage::ScanResult(ScanResult {
                request_id: Uuid::nil(),
                agent_id: agent_id.clone(),
                items: scanned_items,
            }))
            .await?;
        debug!("Initial scan results sent to server");

        info!("Agent session active, listening for server commands");

        loop {
            tokio::select! {
                Some(msg) = self.rx.recv() => {
                    debug!("Sending queued message to server: {:?}", msg);
                    self.connection.send(&msg).await?;
                }

                command_result = self.connection.receive() => {
                    let command = match command_result {
                        Ok(cmd) => cmd,
                        Err(err) => {
                            error!("Connection error while receiving from server: {err}");
                            return Err(err);
                        }
                    };

                    debug!("Received command from server: {:?}", command);
                    self.handle_command(command, &agent_id).await?;
                }
            }
        }
    }

    pub async fn handle_command(
        &mut self,
        command: ServerToAgentMessage,
        agent_id: &str,
    ) -> anyhow::Result<()> {
        match command {
            ServerToAgentMessage::Ping => {
                self.handle_ping().await?;
            }
            ServerToAgentMessage::Launch(cmd) => {
                self.handle_launch(cmd, agent_id).await;
            }
            ServerToAgentMessage::Scan(cmd) => {
                self.handle_scan(cmd, agent_id).await?;
            }
        }
        Ok(())
    }

    pub async fn handle_ping(&mut self) -> anyhow::Result<()> {
        trace!("Ping received from server, responding with Pong");
        self.connection.send(&AgentToServerMessage::Pong).await?;
        Ok(())
    }

    pub async fn handle_launch(&mut self, command: LaunchCommand, agent_id: &str) {
        info!(
            "Received Launch command for media_id='{}', launch_id='{}', program='{}'",
            command.media_id, command.launch_id, command.program
        );

        match launch_program_with_command(&command) {
            Ok(_) => {
                info!(
                    "Program launched successfully for launch_id='{}'",
                    command.launch_id
                );
            }
            Err(err) => {
                error!(
                    "Failed to launch program for launch_id='{}': {err}",
                    command.launch_id
                );
            }
        }

        let tx = self.tx.clone();
        let agent_id = agent_id.to_string();
        tokio::spawn(async move {
            spawn_media_tracker(tx, agent_id, command).await;
        });
    }

    pub async fn handle_scan(
        &mut self,
        command: ScanCommand,
        agent_id: &str,
    ) -> anyhow::Result<()> {
        info!(
            "Received Scan command: request_id={}, media_type={:?}",
            command.request_id, command.media_type
        );

        let items = match command.media_type {
            Some(mt) => self.scanner_registry.scan_media_type(mt).await,
            None => self.scanner_registry.scan_all().await,
        };

        info!(
            "Scan complete for request_id={}: found {} item(s)",
            command.request_id,
            items.len()
        );

        self.connection
            .send(&AgentToServerMessage::ScanResult(ScanResult {
                request_id: command.request_id,
                agent_id: agent_id.to_string(),
                items,
            }))
            .await?;
        debug!(
            "Sent scan results to server for request_id={}",
            command.request_id
        );
        Ok(())
    }
}
