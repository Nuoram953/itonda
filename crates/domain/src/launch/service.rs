use std::{
    path::Path,
    process::{Child, Command},
};

use itonda_database::{
    agent::find_available_agent,
    emulators::queries::find_emulator_by_id,
    error::DatabaseError,
    media::{MediaLaunchRow, find_media_launch_by_id},
};
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::{launch::errors::LaunchError, protocol::server_to_agent::LaunchCommand};

pub async fn get_launch_media_details(
    pool: &SqlitePool,
    launch_id: String,
) -> Result<(LaunchCommand, String), LaunchError> {
    let launch = find_media_launch_by_id(pool, launch_id.clone())
        .await
        .map_err(|err| match err {
            DatabaseError::NotFound => LaunchError::NotFound,
            err => LaunchError::Database(err),
        })?;

    let agent_id = if let Some(target_agent_id) = launch.agent_id.clone() {
        target_agent_id
    } else {
        let connection = find_available_agent(pool)
            .await?
            .ok_or(LaunchError::NoAgentAvailable)?;
        connection.agent_id
    };

    let (program, args, working_directory) = resolve_launch_execution(pool, &launch).await?;

    let command = LaunchCommand {
        program,
        args,
        working_directory,
        request_id: Uuid::new_v4(),
        media_id: launch.media_id,
        launch_id,
    };

    Ok((command, agent_id))
}

pub fn resolve_emulator_fields(
    emulator_program: &str,
    emulator_args: &[String],
    emulator_working_dir: Option<&str>,
    override_program: &str,
    override_args: &[String],
    argument_mode: &str,
    override_working_dir: Option<&str>,
    game_location: Option<&str>,
) -> (String, Vec<String>, Option<String>) {
    let program = if override_program.trim().is_empty() {
        emulator_program.to_string()
    } else {
        override_program.trim().to_string()
    };

    let working_directory = override_working_dir
        .map(str::trim)
        .filter(|dir| !dir.is_empty())
        .map(ToString::to_string)
        .or_else(|| {
            emulator_working_dir
                .map(str::trim)
                .filter(|dir| !dir.is_empty())
                .map(ToString::to_string)
        })
        .or_else(|| derive_working_directory(&program, game_location));

    let mut raw_args = if argument_mode == "replace" {
        override_args.to_vec()
    } else {
        let mut args = emulator_args.to_vec();
        args.extend(override_args.iter().cloned());
        args
    };

    apply_game_location(&mut raw_args, game_location);

    (program, raw_args, working_directory)
}

pub fn derive_working_directory(program: &str, game_location: Option<&str>) -> Option<String> {
    let from_game = game_location
        .map(str::trim)
        .filter(|loc| !loc.is_empty())
        .and_then(|loc| Path::new(loc).parent())
        .map(|p| p.to_string_lossy().into_owned())
        .filter(|s| !s.is_empty());

    from_game.or_else(|| {
        Path::new(program.trim())
            .parent()
            .map(|p| p.to_string_lossy().into_owned())
            .filter(|s| !s.is_empty())
    })
}

fn apply_game_location(args: &mut Vec<String>, game_location: Option<&str>) {
    let Some(game_location) = game_location.map(str::trim).filter(|loc| !loc.is_empty()) else {
        return;
    };

    let mut replaced_placeholder = false;
    for arg in args.iter_mut() {
        if arg.contains("{game}") || arg.contains("{rom}") {
            *arg = arg
                .replace("{game}", game_location)
                .replace("{rom}", game_location);
            replaced_placeholder = true;
        }
    }

    if !replaced_placeholder {
        args.push(game_location.to_string());
    }
}

async fn resolve_launch_execution(
    pool: &SqlitePool,
    launch: &MediaLaunchRow,
) -> Result<(String, Vec<String>, Option<String>), LaunchError> {
    let mut launch_args: Vec<String> = serde_json::from_str(&launch.arguments).unwrap_or_default();

    if let Some(emulator_id) = launch.emulator_id.as_deref()
        && launch.program.trim().is_empty()
    {
        let emulator = find_emulator_by_id(pool, emulator_id).await?;
        let emu_args: Vec<String> = serde_json::from_str(&emulator.arguments).unwrap_or_default();

        return Ok(resolve_emulator_fields(
            &emulator.program,
            &emu_args,
            emulator.working_directory.as_deref(),
            &launch.program,
            &launch_args,
            &launch.argument_mode,
            launch.working_directory.as_deref(),
            launch.game_location.as_deref(),
        ));
    }

    apply_game_location(&mut launch_args, launch.game_location.as_deref());
    // If arguments were already pre-populated with game_location, avoid appending it a second time.
    if let Some(loc) = launch
        .game_location
        .as_deref()
        .map(str::trim)
        .filter(|loc| !loc.is_empty())
    {
        let count = launch_args.iter().filter(|a| a.as_str() == loc).count();
        if count > 1 && launch_args.last().map(String::as_str) == Some(loc) {
            launch_args.pop();
        }
    }

    Ok((
        launch.program.clone(),
        launch_args,
        launch.working_directory.clone(),
    ))
}

pub fn launch_program_with_command(command: &LaunchCommand) -> std::io::Result<Child> {
    let mut cmd = Command::new(&command.program);
    cmd.args(&command.args);
    if let Some(working_dir) = &command.working_directory {
        //TODO: get launch type instead of hardcoding storefront
        let is_storefront =
            command.program == "steam" || command.args.iter().any(|a| a.starts_with("steam://"));
        if !is_storefront {
            cmd.current_dir(working_dir);
        }
    }
    cmd.spawn()
}

#[cfg(test)]
mod tests {
    use super::*;
    use itonda_database::{
        agent::{AgentConnectionsInsert, AgentsInsert, insert_agent_connection, upsert_agent},
        emulators::{models::EmulatorInsert, queries::insert_emulator},
        media::{MediaInsert, MediaLaunchCreate, create_media_launch, insert_media},
        test_utils::setup_db,
    };

    async fn seed_media_and_agent(pool: &SqlitePool) -> (String, String) {
        let media = insert_media(
            pool,
            MediaInsert {
                title: "Super Mario Sunshine".to_string(),
                media_type: "game".to_string(),
                status_id: 1,
                release_date: None,
                description: None,
                summary: None,
            },
        )
        .await
        .unwrap();

        upsert_agent(
            pool,
            AgentsInsert {
                id: "agent-1".to_string(),
                name: "Desktop".to_string(),
                hostname: "desktop".to_string(),
                platform: "linux".to_string(),
                agent_version: "0.1.0".to_string(),
            },
        )
        .await
        .unwrap();
        insert_agent_connection(
            pool,
            AgentConnectionsInsert {
                id: "conn-1".to_string(),
                agent_id: "agent-1".to_string(),
                ip_address: None,
            },
        )
        .await
        .unwrap();

        (media.id, "agent-1".to_string())
    }

    #[tokio::test]
    async fn resolves_emulator_placeholder_and_append_args() {
        let pool = setup_db().await;
        let (media_id, _agent_id) = seed_media_and_agent(&pool).await;

        let emu = insert_emulator(
            &pool,
            EmulatorInsert {
                name: "Dolphin".to_string(),
                program: "/usr/bin/dolphin-emu".to_string(),
                arguments: r#"["-b","-e","{game}"]"#.to_string(),
                options: "[]".to_string(),
                working_directory: Some("/games/gc".to_string()),
            },
        )
        .await
        .unwrap();

        let launch = create_media_launch(
            &pool,
            MediaLaunchCreate {
                media_id,
                agent_id: None,
                name: "Dolphin".to_string(),
                launch_type: "emulator".to_string(),
                emulator_id: Some(emu.id),
                program: String::new(),
                arguments: r#"["--fullscreen"]"#.to_string(),
                argument_mode: "append".to_string(),
                working_directory: None,
                game_location: Some("/roms/sunshine.iso".to_string()),
                is_default: true,
                enabled: true,
            },
        )
        .await
        .unwrap();

        let (cmd, agent) = get_launch_media_details(&pool, launch.id).await.unwrap();
        assert_eq!(agent, "agent-1");
        assert_eq!(cmd.program, "/usr/bin/dolphin-emu");
        assert_eq!(
            cmd.args,
            vec!["-b", "-e", "/roms/sunshine.iso", "--fullscreen"]
        );
        assert_eq!(cmd.working_directory.as_deref(), Some("/games/gc"));
    }

    #[tokio::test]
    async fn resolves_emulator_overrides_replace_and_fallback_append_game_location() {
        let pool = setup_db().await;
        let (media_id, agent_id) = seed_media_and_agent(&pool).await;

        let emu = insert_emulator(
            &pool,
            EmulatorInsert {
                name: "PCSX2".to_string(),
                program: "/usr/bin/pcsx2".to_string(),
                arguments: r#"["-fullscreen"]"#.to_string(),
                options: "[]".to_string(),
                working_directory: Some("/games/ps2".to_string()),
            },
        )
        .await
        .unwrap();

        let launch = create_media_launch(
            &pool,
            MediaLaunchCreate {
                media_id,
                agent_id: Some(agent_id.clone()),
                name: "PCSX2 Custom".to_string(),
                launch_type: "emulator".to_string(),
                emulator_id: Some(emu.id),
                program: "/opt/pcsx2-qt".to_string(),
                arguments: r#"["--nogui"]"#.to_string(),
                argument_mode: "replace".to_string(),
                working_directory: Some("/custom/dir".to_string()),
                game_location: Some("/roms/ffx.iso".to_string()),
                is_default: true,
                enabled: true,
            },
        )
        .await
        .unwrap();

        let (cmd, agent) = get_launch_media_details(&pool, launch.id).await.unwrap();
        assert_eq!(agent, agent_id);
        assert_eq!(cmd.program, "/opt/pcsx2-qt");
        assert_eq!(cmd.args, vec!["--nogui", "/roms/ffx.iso"]);
        assert_eq!(cmd.working_directory.as_deref(), Some("/custom/dir"));
    }

    #[test]
    fn derives_working_directory_from_program_or_game_location() {
        assert_eq!(
            derive_working_directory("/opt/dolphin/dolphin-emu", Some("/roms/gc/sunshine.iso"))
                .as_deref(),
            Some("/roms/gc")
        );
        assert_eq!(
            derive_working_directory("/opt/dolphin/dolphin-emu", None).as_deref(),
            Some("/opt/dolphin")
        );
        assert_eq!(derive_working_directory("dolphin-emu", None), None);
    }
}
