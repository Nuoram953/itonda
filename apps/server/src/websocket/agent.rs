use axum::{
    extract::{
        State,
        ws::{Message, WebSocket, WebSocketUpgrade},
    },
    response::Response,
};
use itonda_database::{
    agent::{
        AgentConnectionsInsert, AgentsInsert, disconnect_agent_connection, insert_agent_connection,
        upsert_agent,
    },
    media::{
        MediaInstallationUpsert, MediaLaunchSessionInsert, MediaLaunchUpsert,
        delete_stale_agent_installations, upsert_media_installation, upsert_media_launch,
    },
};
pub use itonda_domain::agents::AgentManager;
use itonda_domain::{
    events::{AgentEvent, AppEvent, EventBus, MediaEvent},
    media::{
        models::ExternalIdProvider,
        service::{find_or_create_media, update_playtime},
    },
    protocol::{AgentRegistration, AgentToServerMessage, ScanResult, ServerToAgentMessage},
    storefronts::models::StorefrontId,
};
use sqlx::SqlitePool;
use tokio::sync::mpsc::{self, Sender};
use tracing::{debug, info, warn};
use uuid::Uuid;

use crate::{state::AppState, workers::jobs::Job};

pub async fn agent_ws(ws: WebSocketUpgrade, State(state): State<AppState>) -> Response {
    let manager = state.agent_manager.clone();
    let events = state.events.clone();
    let pool = state.db.clone();
    let jobs = state.jobs.clone();

    ws.on_upgrade(move |socket| async move {
        if let Err(err) = handle_agent(socket, &pool, manager, events, jobs).await {
            tracing::error!("Agent connection error: {err}");
        }
    })
}

async fn handle_agent(
    mut socket: WebSocket,
    pool: &SqlitePool,
    agent_manager: AgentManager,
    events: EventBus,
    jobs: Sender<Job>,
) -> anyhow::Result<()> {
    debug!("Waiting for registration");

    let registration = wait_for_registration(&mut socket).await?;

    debug!("Registered agent: {:?}", registration);

    let agent_id = registration.id.clone();

    upsert_agent(
        pool,
        AgentsInsert {
            id: registration.id,
            name: registration.name,
            hostname: registration.hostname,
            platform: registration.platform,
            agent_version: registration.agent_version,
        },
    )
    .await?;

    let _ = disconnect_agent_connection(pool, agent_id.clone()).await;

    insert_agent_connection(
        pool,
        AgentConnectionsInsert {
            id: Uuid::new_v4().into(),
            agent_id: agent_id.clone(),
            ip_address: Some(registration.ip_address),
        },
    )
    .await?;

    let (tx, mut rx) = mpsc::channel(32);

    agent_manager.register(agent_id.clone(), tx).await;

    if let Ok(agent_uuid) = Uuid::parse_str(&agent_id) {
        events.publish(AppEvent::Agent(AgentEvent::Connected {
            agent_id: agent_uuid,
        }));
    }

    let result = run_agent_loop(&mut socket, &mut rx, pool, &events, &agent_id, &jobs).await;

    let _ = disconnect_agent_connection(pool, agent_id.clone()).await;
    agent_manager.unregister(&agent_id).await;

    if let Ok(agent_uuid) = Uuid::parse_str(&agent_id) {
        events.publish(AppEvent::Agent(AgentEvent::Disconnected {
            agent_id: agent_uuid,
        }));
    }

    result
}

async fn run_agent_loop(
    socket: &mut WebSocket,
    rx: &mut mpsc::Receiver<ServerToAgentMessage>,
    pool: &SqlitePool,
    events: &EventBus,
    agent_id: &str,
    _jobs: &Sender<Job>,
) -> anyhow::Result<()> {
    loop {
        tokio::select! {
            Some(command) = rx.recv() => {
                info!(?command, %agent_id, "Event published to agent");

                let message = serde_json::to_string(&command).unwrap();

                socket
                    .send(axum::extract::ws::Message::Text(message.into()))
                    .await?;
            }

            Some(message) = socket.recv() => {
                debug!("Received from agent: {:?}", message);
                match message {
                    Ok(Message::Text(text)) => {
                        if let Ok(agent_msg) = serde_json::from_str::<AgentToServerMessage>(&text) {
                            match agent_msg {
                                AgentToServerMessage::ScanResult(scan_result) => {
                                    if let Err(err) = handle_agent_scan_result(pool, events, scan_result).await {
                                        warn!("Error handling scan result: {err}");
                                    }
                                }
                                AgentToServerMessage::Pong => {
                                    debug!("Pong received from agent {}", agent_id);
                                }
                                AgentToServerMessage::MediaStarted(payload) => {
                                    events.publish(AppEvent::Media(MediaEvent::Launched { media_id:payload.media_id, launch_id: payload.launch_id, agent_id:payload.agent_id }));
                                }

                                AgentToServerMessage::MediaStopped(payload) => {
                                    events.publish(AppEvent::Media(MediaEvent::Stopped { media_id:payload.media_id, launch_id: payload.launch_id.clone(), agent_id:payload.agent_id, duration_seconds: payload.duration_seconds }));

                                    update_playtime(pool, MediaLaunchSessionInsert{
                                        launch_id: payload.launch_id,
                                        started_at: payload.started_at.to_string(),
                                        completed_at: payload.stopped_at.to_string(),
                                        duration_seconds: payload.duration_seconds.to_string()
                                    }).await?;

                                }
                                _ => {}
                            }
                        }
                    }
                    Ok(_) => {}
                    Err(err) => {
                        warn!("Error message: {:?}", err);
                        disconnect_agent_connection(pool, agent_id.to_string()).await?;
                        anyhow::bail!("Agent socket error: {err}");
                    }
                }
            }
            else => {
                break;
            }
        }
    }

    Ok(())
}

pub async fn handle_agent_scan_result(
    pool: &SqlitePool,
    events: &EventBus,
    scan_result: ScanResult,
) -> anyhow::Result<()> {
    info!(
        "Processing scan result with {} items from agent {}",
        scan_result.items.len(),
        scan_result.agent_id
    );

    if let Ok(agent_uuid) = Uuid::parse_str(&scan_result.agent_id) {
        events.publish(AppEvent::Agent(AgentEvent::ScanStarted {
            agent_id: agent_uuid,
        }));
    }

    let mut active_installation_ids = Vec::with_capacity(scan_result.items.len());

    for item in scan_result.items {
        let storefront_id = StorefrontId::try_from(item.source.as_str()).ok();
        let storefront_db_id = storefront_id
            .as_ref()
            .map(|sf| sf.as_str())
            .unwrap_or(item.source.as_str());

        let ext_provider = storefront_id.map(|sf| match sf {
            StorefrontId::Steam => ExternalIdProvider::Steam,
        });

        let media_row = find_or_create_media(
            pool,
            &item.title,
            item.media_type,
            if item.source.is_empty() {
                None
            } else {
                Some(storefront_db_id)
            },
            ext_provider.map(|p| p.as_str()),
            item.external_id.as_deref(),
        )
        .await?;

        let install_path = item.working_directory.clone().or_else(|| {
            item.launch
                .as_ref()
                .and_then(|l| l.working_directory.clone())
        });

        let installation = upsert_media_installation(
            pool,
            MediaInstallationUpsert {
                media_id: media_row.id.clone(),
                agent_id: scan_result.agent_id.clone(),
                storefront_id: if item.source.is_empty() {
                    None
                } else {
                    Some(storefront_db_id.to_string())
                },
                external_id: item.external_id.clone(),
                path: install_path,
            },
        )
        .await?;
        active_installation_ids.push(installation.value.id);

        if let Some(launch) = item.launch {
            upsert_media_launch(
                pool,
                MediaLaunchUpsert {
                    media_id: media_row.id,
                    agent_id: Some(scan_result.agent_id.clone()),
                    name: launch.name,
                    launch_type: launch.launch_type.as_str().into(),
                    program: launch.program,
                    arguments: serde_json::to_string(&launch.arguments)?,
                    working_directory: launch.working_directory,
                    is_default: false,
                    enabled: true,
                },
            )
            .await?;
        }
    }

    let removed =
        delete_stale_agent_installations(pool, &scan_result.agent_id, &active_installation_ids)
            .await?;
    if removed > 0 {
        info!(
            "Removed {} uninstalled media installation(s) for agent {}",
            removed, scan_result.agent_id
        );
    }

    if let Ok(agent_uuid) = Uuid::parse_str(&scan_result.agent_id) {
        events.publish(AppEvent::Agent(AgentEvent::ScanCompleted {
            agent_id: agent_uuid,
        }));
    }

    info!(
        "Scan result processing complete for agent {}",
        scan_result.agent_id
    );
    Ok(())
}

async fn wait_for_registration(socket: &mut WebSocket) -> anyhow::Result<AgentRegistration> {
    while let Some(message) = socket.recv().await {
        match message? {
            Message::Text(text) => {
                debug!("Raw agent message: {}", text);
                let message: AgentToServerMessage = serde_json::from_str(&text)?;

                debug!("Parsed: {:?}", message);

                if let AgentToServerMessage::Register(registration) = message {
                    return Ok(registration);
                }
                // match message {
                //     AgentToServerMessage::Register(registration) => return Ok(registration),
                //     _ => {}
                // }
            }

            Message::Close(_) => {
                anyhow::bail!("Agent disconnected before registering");
            }

            _ => {}
        }
    }

    anyhow::bail!("Socket closed");
}

#[cfg(test)]
mod tests {
    use itonda_database::{
        agent::{AgentsInsert, upsert_agent},
        media::{find_installations_by_media_id, find_media_by_title},
        test_utils::setup_db,
    };
    use itonda_domain::{
        events::EventBus,
        media::types::{MediaLaunchType, MediaType},
        protocol::ScanResult,
        scanner::models::{ScannedLaunch, ScannedMedia},
    };
    use uuid::Uuid;

    use super::handle_agent_scan_result;

    fn scanned_game(title: &str, app_id: &str, dir: &str) -> ScannedMedia {
        ScannedMedia {
            media_type: MediaType::Game,
            title: title.into(),
            external_id: Some(app_id.into()),
            source: "0".into(),
            working_directory: Some(dir.into()),
            launch: Some(ScannedLaunch {
                name: "Steam".into(),
                launch_type: MediaLaunchType::Storefront,
                program: "steam".into(),
                arguments: vec![format!("steam://run/{app_id}")],
                working_directory: Some(dir.into()),
            }),
        }
    }

    #[tokio::test]
    async fn handle_agent_scan_result_removes_uninstalled_games() {
        let pool = setup_db().await;
        let events = EventBus::new();

        for agent_id in ["agent-1", "agent-2"] {
            upsert_agent(
                &pool,
                AgentsInsert {
                    id: agent_id.into(),
                    name: format!("Agent {agent_id}"),
                    hostname: "desktop".into(),
                    platform: "linux".into(),
                    agent_version: "1.0.0".into(),
                },
            )
            .await
            .unwrap();
        }

        handle_agent_scan_result(
            &pool,
            &events,
            ScanResult {
                request_id: Uuid::new_v4(),
                agent_id: "agent-1".into(),
                items: vec![
                    scanned_game("Portal 2", "620", "/games/portal2"),
                    scanned_game("Half-Life 2", "220", "/games/hl2"),
                ],
            },
        )
        .await
        .unwrap();

        handle_agent_scan_result(
            &pool,
            &events,
            ScanResult {
                request_id: Uuid::new_v4(),
                agent_id: "agent-2".into(),
                items: vec![scanned_game("Half-Life 2", "220", "/games/hl2")],
            },
        )
        .await
        .unwrap();

        let portal2 = find_media_by_title(&pool, "Portal 2".into())
            .await
            .unwrap()
            .unwrap();
        let hl2 = find_media_by_title(&pool, "Half-Life 2".into())
            .await
            .unwrap()
            .unwrap();

        assert_eq!(
            find_installations_by_media_id(&pool, &portal2.id)
                .await
                .unwrap()
                .len(),
            1
        );
        assert_eq!(
            find_installations_by_media_id(&pool, &hl2.id)
                .await
                .unwrap()
                .len(),
            2
        );

        handle_agent_scan_result(
            &pool,
            &events,
            ScanResult {
                request_id: Uuid::new_v4(),
                agent_id: "agent-1".into(),
                items: vec![scanned_game("Portal 2", "620", "/games/portal2")],
            },
        )
        .await
        .unwrap();

        let portal2_installs = find_installations_by_media_id(&pool, &portal2.id)
            .await
            .unwrap();
        assert_eq!(portal2_installs.len(), 1);
        assert_eq!(portal2_installs[0].agent_id, "agent-1");

        let hl2_installs = find_installations_by_media_id(&pool, &hl2.id)
            .await
            .unwrap();
        assert_eq!(hl2_installs.len(), 1);
        assert_eq!(hl2_installs[0].agent_id, "agent-2");

        handle_agent_scan_result(
            &pool,
            &events,
            ScanResult {
                request_id: Uuid::new_v4(),
                agent_id: "agent-1".into(),
                items: vec![],
            },
        )
        .await
        .unwrap();

        assert!(
            find_installations_by_media_id(&pool, &portal2.id)
                .await
                .unwrap()
                .is_empty()
        );
        assert_eq!(
            find_installations_by_media_id(&pool, &hl2.id)
                .await
                .unwrap()
                .len(),
            1
        );
    }
}
