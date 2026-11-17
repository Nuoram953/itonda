use axum::{
    Json,
    extract::{Query, State},
};
use itonda_domain::agents::service::get_active_agents;
use tracing::instrument;

use crate::{
    api::{
        agents::schemas::{
            AgentDirectoryListingResponse, GetAgentsResponse, ListAgentFilesQuery,
        },
        error::ApiError,
    },
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

#[utoipa::path(
    get,
    path = "/agents/fs",
    params(ListAgentFilesQuery),
    responses(
        (status = 200, body = AgentDirectoryListingResponse),
    )
)]
#[instrument(skip(state))]
pub async fn list_agent_files(
    State(state): State<AppState>,
    Query(query): Query<ListAgentFilesQuery>,
) -> Result<Json<AgentDirectoryListingResponse>, ApiError> {
    let listing = state
        .agent_manager
        .list_directory(query.agent_id.as_deref(), query.path)
        .await?;

    Ok(Json(AgentDirectoryListingResponse {
        path: listing.path,
        parent: listing.parent,
        entries: listing.entries,
        error: listing.error,
    }))
}
