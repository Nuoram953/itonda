use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use itonda_domain::media::service::get_all_media;
use tracing::instrument;
use uuid::Uuid;

use crate::{
    api::{
        error::ApiError,
        extractor::AppJson,
        media::schemas::{MediaImportPayload, MediaResponse},
        response::{JobResponse, JobStatus},
    },
    state::AppState,
    workers::jobs::{ImportItem, ImportJob, Job},
};

#[utoipa::path(
    get,
    path = "/media",
    responses(
        (
            status = 200,
            body = MediaResponse
        )
    )
)]
#[instrument(skip(state))]
pub async fn get_media(State(state): State<AppState>) -> Result<Json<MediaResponse>, ApiError> {
    let media = get_all_media(&state.db).await;

    // // state
    // //     .agent_manager
    // //     .send("test uuid", AgentMessage::Ping)
    // //     .await
    // //     .unwrap();
    // //
    // let secrets = state.secrets.get().await;
    // let steam = SteamStorefront::new(
    //     secrets.storefronts.steam.api_key,
    //     secrets.storefronts.steam.steam_id,
    // );
    //
    // let games = steam.owned_games().await.unwrap();
    //
    // println!("steam call {:?}", games);

    Ok(Json(MediaResponse {
        items: media.unwrap(),
    }))
}

#[utoipa::path(
    post,
    path = "/media/import",
    request_body = MediaImportPayload,
    responses(
        (
            status = 202,
            body = JobResponse
        )
    )
)]
#[instrument(skip(state, request))]
pub async fn import_media(
    State(state): State<AppState>,
    AppJson(request): AppJson<MediaImportPayload>,
) -> Result<impl IntoResponse, ApiError> {
    let job_id = Uuid::new_v4();

    let items = request
        .items
        .into_iter()
        .map(|item| ImportItem {
            title: item.title,
            media_type: item.media_type,
        })
        .collect();

    state
        .jobs
        .send(Job::Import(ImportJob { id: job_id, items }))
        .await
        .map_err(|err| {
            tracing::error!(?err, "Failed to queue import job");
            ApiError::WorkerUnavailable
        })?;

    Ok((
        StatusCode::ACCEPTED,
        Json(JobResponse {
            job_id: job_id.to_string(),
            status: JobStatus::Queued,
        }),
    ))
}
