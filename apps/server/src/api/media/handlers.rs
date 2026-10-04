use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use itonda_domain::{
    launch::service::get_launch_media_details,
    media::{
        models::{ExternalIdProvider, Media},
        service as MediaService,
        types::{MediaStatus, MediaType},
    },
    metadata::models::MediaSearchResult,
    protocol::ServerToAgentMessage,
};
use tracing::instrument;
use uuid::Uuid;

use crate::{
    api::{
        error::ApiError,
        extractor::AppJson,
        media::schemas::{
            CreateMediaPayload, MediaImportPayload, MediaQueryParams, MediaRefreshPayload,
            MediaResponse, MediaSearchQueryParams,
        },
        response::{CommandResponse, CommandStatus, JobResponse, JobStatus},
    },
    state::AppState,
    workers::jobs::{ImportItem, ImportJob, Job, SyncJob},
};

#[utoipa::path(
    get,
    path = "/media",
    params(
        MediaQueryParams
    ),
    responses(
        (
            status = 200,
            body = MediaResponse
        )
    )
)]
#[instrument(skip(state))]
pub async fn get_media(
    State(state): State<AppState>,
    Query(query): Query<MediaQueryParams>,
) -> Result<Json<MediaResponse>, ApiError> {
    let paginated = MediaService::get_paginated_media(
        &state.db,
        MediaService::MediaSearchQuery {
            media_type: query.media_type,
            search: query.search.as_deref(),
            status: query.status,
            storefront: query.storefront.as_deref(),
            sort_by: query.sort_by,
            sort_order: query.sort_order,
            page: query.page,
            limit: query.limit,
        },
    )
    .await?;

    Ok(Json(MediaResponse {
        items: paginated.items,
        total: paginated.total,
        page: paginated.page,
        limit: paginated.limit,
        total_pages: paginated.total_pages,
        has_next: paginated.has_next,
    }))
}

#[utoipa::path(
    get,
    path = "/media/{media_id}",
    params(
        ("media_id" = String, Path, description = "Media ID"),
    ),
    responses(
        (
            status = 200,
            body = Media
        )
    )
)]
pub async fn get_media_by_id(
    State(state): State<AppState>,
    Path(media_id): Path<String>,
) -> Result<Json<Media>, ApiError> {
    let media = MediaService::get_media_by_id(&state.db, media_id).await?;

    Ok(Json(media))
}

#[utoipa::path(
    post,
    path = "/media/refresh",
    request_body = MediaRefreshPayload,
    responses(
        (
            status = 202,
            body = JobResponse
        )
    )
)]
#[instrument(skip(state, request))]
pub async fn refresh(
    State(state): State<AppState>,
    AppJson(request): AppJson<MediaRefreshPayload>,
) -> Result<impl IntoResponse, ApiError> {
    let job_id = Uuid::new_v4();

    state
        .jobs
        .send(Job::Sync(SyncJob {
            id: job_id,
            storefront: request.storefront,
            media_id: None,
            force: request.force,
        }))
        .await
        .map_err(|err| {
            tracing::error!(?err, "Failed to queue sync job");
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

#[utoipa::path(
    post,
    path = "/media/launch/{launch_id}",
    params(
        ("launch_id" = String, Path, description = "Launch profile id")
    ),
    responses(
        (
            status = 202,
            body = CommandResponse
        )
    )
)]
#[instrument(skip(state))]
pub async fn launch_media(
    State(state): State<AppState>,
    Path(launch_id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    let (command, agent_id) = get_launch_media_details(&state.db, launch_id).await?;

    let command_id = Uuid::new_v4().to_string();

    let _ = state
        .agent_manager
        .send(&agent_id, ServerToAgentMessage::Launch(command))
        .await;

    Ok((
        StatusCode::ACCEPTED,
        Json(CommandResponse {
            id: command_id,
            command: "launch".to_string(),
            status: CommandStatus::Accepted,
        }),
    ))
}

#[utoipa::path(
    patch,
    path = "/media/{media_id}/status/{status_id}",
    params(
        ("media_id" = String, Path, description = "ID of the media"),
        ("status_id" = MediaStatus, Path, description = "ID of new status")
    ),
    responses(
        (
            status = 204,
        )
    )
)]
#[instrument(skip(state))]
pub async fn update_status(
    State(state): State<AppState>,
    Path((media_id, status_id)): Path<(String, MediaStatus)>,
) -> Result<impl IntoResponse, ApiError> {
    MediaService::update_status(&state.db, media_id, status_id).await?;

    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(
    get,
    path = "/media/search",
    params(
        MediaSearchQueryParams
    ),
    responses(
        (
            status = 200,
            body = Vec<MediaSearchResult>
        )
    )
)]
#[instrument(skip(state))]
pub async fn search_media(
    State(state): State<AppState>,
    Query(query): Query<MediaSearchQueryParams>,
) -> Result<Json<Vec<MediaSearchResult>>, ApiError> {
    if query.query.trim().len() < 2 {
        return Ok(Json(Vec::new()));
    }

    let results = state
        .metadata
        .search(query.query.trim(), query.media_type)
        .await
        .map_err(|err| {
            tracing::error!(?err, "Failed to search media catalog");
            ApiError::InternalServer
        })?;

    Ok(Json(results))
}

#[utoipa::path(
    post,
    path = "/media",
    request_body = CreateMediaPayload,
    responses(
        (
            status = 201,
            body = Media
        )
    )
)]
#[instrument(skip(state, request))]
pub async fn create_media(
    State(state): State<AppState>,
    AppJson(request): AppJson<CreateMediaPayload>,
) -> Result<impl IntoResponse, ApiError> {
    if request.title.trim().is_empty() {
        return Err(ApiError::Validation("title cannot be empty".to_string()));
    }

    let inserted = itonda_database::media::insert_media(
        &state.db,
        itonda_database::media::MediaInsert {
            title: request.title.trim().to_string(),
            media_type: request.media_type.as_str().to_string(),
            status_id: MediaStatus::NotStarted.id(),
            ..Default::default()
        },
    )
    .await?;

    if let Some(external_id) = request.external_id
        && !external_id.trim().is_empty()
    {
        let provider = match request.media_type {
            MediaType::Game => ExternalIdProvider::Igdb,
            MediaType::Movie | MediaType::TvShow => ExternalIdProvider::Tmdb,
        };

        let _ = itonda_database::media::upsert_media_external_id(
            &state.db,
            itonda_database::media::MediaExternalIdUpsert {
                media_id: inserted.id.clone(),
                provider: provider.as_str().to_string(),
                external_id: external_id.trim().to_string(),
            },
        )
        .await;
    }

    let sync_job_id = Uuid::new_v4();
    let _ = state
        .jobs
        .send(Job::Sync(SyncJob {
            id: sync_job_id,
            storefront: None,
            media_id: Some(inserted.id.clone()),
            force: false,
        }))
        .await;

    let media = MediaService::get_media_by_id(&state.db, inserted.id).await?;

    Ok((StatusCode::CREATED, Json(media)))
}
