use axum::{Json, extract::State};
use itonda_domain::agents::service::get_active_agents;
use tracing::instrument;

use crate::{
    api::{agents::schemas::GetAgentsResponse, error::ApiError},
    state::AppState,
};

#[utoipa::path(
    get,
    path = "/agents",
    responses(
        (status = 200, body = GetAgentsResponse),
    )
)]
#[instrument(skip(state))]
pub async fn get_connected_agents(
    State(state): State<AppState>,
) -> Result<Json<GetAgentsResponse>, ApiError> {
    let agents = get_active_agents(&state.db).await?;

    Ok(Json(GetAgentsResponse { agents }))
}
