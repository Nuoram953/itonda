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

    let connection = find_available_agent(pool)
        .await?
        .ok_or(LaunchError::NoAgentAvailable)?;

    let command = LaunchCommand {
        program: launch.program,
        args: serde_json::from_str(&launch.arguments).unwrap(),
        request_id: Uuid::new_v4(),
        media_id: launch.media_id,
    };

    Ok((command, connection.agent_id))
}

pub fn launch_program_with_command(command: &LaunchCommand) -> std::io::Result<Child> {
    Command::new(&command.program).args(&command.args).spawn()
}
