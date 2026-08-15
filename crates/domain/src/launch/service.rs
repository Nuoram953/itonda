use std::process::{Child, Command};

use itonda_database::{
    agent::find_available_agent, error::DatabaseError, media::find_media_launch_by_id,
};
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::{launch::errors::LaunchError, protocol::server_to_agent::LaunchCommand};

pub async fn get_launch_media_details(
    pool: &SqlitePool,
    launch_id: String,
) -> Result<(LaunchCommand, String), LaunchError> {
    let launch = find_media_launch_by_id(pool, launch_id)
        .await
        .map_err(|err| match err {
            DatabaseError::NotFound => LaunchError::NotFound,
            err => LaunchError::Database(err),
        })?;

    let agent_id = if let Some(target_agent_id) = launch.agent_id {
        target_agent_id
    } else {
        let connection = find_available_agent(pool)
            .await?
            .ok_or(LaunchError::NoAgentAvailable)?;
        connection.agent_id
    };

    let command = LaunchCommand {
        program: launch.program,
        args: serde_json::from_str(&launch.arguments).unwrap_or_default(),
        working_directory: launch.working_directory,
        request_id: Uuid::new_v4(),
        media_id: launch.media_id,
    };

    Ok((command, agent_id))
}

pub fn launch_program_with_command(command: &LaunchCommand) -> std::io::Result<Child> {
    let mut cmd = Command::new(&command.program);
    cmd.args(&command.args);
    if let Some(working_dir) = &command.working_directory {
        cmd.current_dir(working_dir);
    }
    cmd.spawn()
}
