use itonda_database::agent::get_agents_with_status;
use sqlx::SqlitePool;

use crate::agents::{errors::AgentsError, models::Agent};

pub async fn get_active_agents(pool: &SqlitePool) -> Result<Vec<Agent>, AgentsError> {
    let rows = get_agents_with_status(pool).await?;

    let agents = rows
        .into_iter()
        .map(Agent::try_from)
        .collect::<Result<Vec<_>, _>>()?;

    Ok(agents)
}
