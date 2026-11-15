use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use itonda_database::{
    emulators::queries::find_emulator_by_id,
    media::{
        MediaLaunchCreate, MediaLaunchUpdate,
        create_media_launch as db_create_media_launch,
        delete_media_launch as db_delete_media_launch, find_media_launch_by_id,
        update_media_launch as db_update_media_launch,
    },
};
use itonda_domain::{
    emulators::errors::EmulatorError,
    launch::service::{derive_working_directory, get_launch_media_details, resolve_emulator_fields},
    media::{
        models::{ExternalIdProvider, Launch, Media},
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
            CreateMediaLaunchPayload, CreateMediaPayload, MediaImportPayload, MediaItemSyncPayload,
            MediaQueryParams, MediaResponse, MediaSearchQueryParams, MediaSyncPayload,
            UpdateMediaLaunchPayload,
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
    path = "/media/sync",
    request_body = Option<MediaSyncPayload>,
    responses(
        (
            status = 202,
            body = JobResponse
        )
    )
)]
#[instrument(skip(state, request))]
pub async fn sync_library(
    State(state): State<AppState>,
    request: Option<Json<MediaSyncPayload>>,
) -> Result<impl IntoResponse, ApiError> {
    let payload = request.map(|Json(r)| r).unwrap_or_default();
    let job_id = Uuid::new_v4();

    state
        .jobs
        .send(Job::Sync(SyncJob {
            id: job_id,
            storefront: payload.storefront,
            media_id: None,
            force: payload.force,
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
    path = "/media/{media_id}/sync",
    params(
        ("media_id" = String, Path, description = "ID of the media to sync")
    ),
    request_body = Option<MediaItemSyncPayload>,
    responses(
        (
            status = 202,
            body = JobResponse
        )
    )
)]
#[instrument(skip(state, request))]
pub async fn sync_media_by_id(
    State(state): State<AppState>,
    Path(media_id): Path<String>,
    request: Option<Json<MediaItemSyncPayload>>,
) -> Result<impl IntoResponse, ApiError> {
    let force = request.map(|Json(r)| r.force).unwrap_or(true);
    let job_id = Uuid::new_v4();

    state
        .jobs
        .send(Job::Sync(SyncJob {
            id: job_id,
            storefront: None,
            media_id: Some(media_id),
            force,
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
        let is_youtube = itonda_domain::sources::youtube::client::YoutubeClient::extract_video_id(
            external_id.trim(),
        )
        .is_some();
        let provider = if is_youtube {
            ExternalIdProvider::Youtube
        } else {
            match request.media_type {
                MediaType::Game => ExternalIdProvider::Igdb,
                MediaType::Movie | MediaType::TvShow => ExternalIdProvider::Tmdb,
            }
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

#[utoipa::path(
    post,
    path = "/media/{media_id}/launches",
    params(
        ("media_id" = String, Path, description = "ID of the media")
    ),
    request_body = CreateMediaLaunchPayload,
    responses(
        (status = 201, body = Launch, description = "Create a launch profile for a media item")
    )
)]
#[instrument(skip(state))]
pub async fn create_media_launch(
    State(state): State<AppState>,
    Path(media_id): Path<String>,
    Json(payload): Json<CreateMediaLaunchPayload>,
) -> Result<impl IntoResponse, ApiError> {
    let _ = MediaService::get_media_by_id(&state.db, media_id.clone()).await?;

    let name = payload.name.trim().to_string();
    if name.is_empty() {
        return Err(ApiError::validation("Launch profile name cannot be empty"));
    }

    if payload.argument_mode != "append" && payload.argument_mode != "replace" {
        return Err(ApiError::validation(
            "argument_mode must be 'append' or 'replace'",
        ));
    }

    let emulator_id = payload
        .emulator_id
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty());

    let emulator = if let Some(ref emu_id) = emulator_id {
        Some(
            find_emulator_by_id(&state.db, emu_id)
                .await
                .map_err(EmulatorError::from)?,
        )
    } else {
        None
    };

    let override_program = payload.program.trim().to_string();
    if emulator.is_none() && override_program.is_empty() {
        return Err(ApiError::validation(
            "Program cannot be empty for non-emulator launch profiles",
        ));
    }

    let game_location = payload
        .game_location
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty());

    if emulator.is_some() && game_location.is_none() {
        return Err(ApiError::validation(
            "Game location is required for emulator launch profiles",
        ));
    }

    let override_working_directory = payload
        .working_directory
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty());

    let (program, resolved_args, working_directory) = if let Some(ref emu) = emulator {
        let emu_args: Vec<String> = serde_json::from_str(&emu.arguments).unwrap_or_default();
        resolve_emulator_fields(
            &emu.program,
            &emu_args,
            emu.working_directory.as_deref(),
            &override_program,
            &payload.arguments,
            &payload.argument_mode,
            override_working_directory.as_deref(),
            game_location.as_deref(),
        )
    } else {
        let working_dir = override_working_directory
            .or_else(|| derive_working_directory(&override_program, game_location.as_deref()));
        (override_program, payload.arguments, working_dir)
    };

    let agent_id = payload
        .agent_id
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty());

    let arguments = serde_json::to_string(&resolved_args).unwrap_or_else(|_| "[]".to_string());

    let row = db_create_media_launch(
        &state.db,
        MediaLaunchCreate {
            media_id,
            agent_id,
            name,
            launch_type: payload.launch_type.trim().to_string(),
            emulator_id,
            program,
            arguments,
            argument_mode: payload.argument_mode,
            working_directory,
            game_location,
            is_default: payload.is_default,
            enabled: payload.enabled,
        },
    )
    .await?;

    Ok((StatusCode::CREATED, Json(Launch::from(row))))
}

#[utoipa::path(
    patch,
    path = "/media/{media_id}/launches/{launch_id}",
    params(
        ("media_id" = String, Path, description = "ID of the media"),
        ("launch_id" = String, Path, description = "ID of the launch profile")
    ),
    request_body = UpdateMediaLaunchPayload,
    responses(
        (status = 200, body = Launch, description = "Update a launch profile")
    )
)]
#[instrument(skip(state))]
pub async fn update_media_launch(
    State(state): State<AppState>,
    Path((media_id, launch_id)): Path<(String, String)>,
    Json(payload): Json<UpdateMediaLaunchPayload>,
) -> Result<Json<Launch>, ApiError> {
    let existing = find_media_launch_by_id(&state.db, launch_id.clone())
        .await
        .map_err(|_| ApiError::not_found("Launch profile not found"))?;

    if existing.media_id != media_id {
        return Err(ApiError::not_found("Launch profile not found"));
    }

    let name = match payload.name {
        Some(n) => {
            let trimmed = n.trim().to_string();
            if trimmed.is_empty() {
                return Err(ApiError::validation("Launch profile name cannot be empty"));
            }
            trimmed
        }
        None => existing.name,
    };

    let launch_type = payload
        .launch_type
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or(existing.launch_type);

    let emulator_id = match payload.emulator_id {
        Some(opt) => opt.map(|s| s.trim().to_string()).filter(|s| !s.is_empty()),
        None => existing.emulator_id,
    };

    if let Some(ref emu_id) = emulator_id {
        find_emulator_by_id(&state.db, emu_id)
            .await
            .map_err(EmulatorError::from)?;
    }

    let argument_mode = match payload.argument_mode {
        Some(mode) => {
            if mode != "append" && mode != "replace" {
                return Err(ApiError::validation(
                    "argument_mode must be 'append' or 'replace'",
                ));
            }
            mode
        }
        None => existing.argument_mode,
    };

    let program = payload
        .program
        .map(|s| s.trim().to_string())
        .unwrap_or(existing.program);

    if emulator_id.is_none() && program.is_empty() {
        return Err(ApiError::validation(
            "Program cannot be empty for non-emulator launch profiles",
        ));
    }

    let game_location = match payload.game_location {
        Some(opt) => opt.map(|s| s.trim().to_string()).filter(|s| !s.is_empty()),
        None => existing.game_location,
    };

    if emulator_id.is_some() && game_location.is_none() {
        return Err(ApiError::validation(
            "Game location is required for emulator launch profiles",
        ));
    }

    let working_directory = match payload.working_directory {
        Some(opt) => opt.map(|s| s.trim().to_string()).filter(|s| !s.is_empty()),
        None => existing.working_directory,
    };

    let agent_id = match payload.agent_id {
        Some(opt) => opt.map(|s| s.trim().to_string()).filter(|s| !s.is_empty()),
        None => existing.agent_id,
    };

    let arguments = match payload.arguments {
        Some(args) => serde_json::to_string(&args).unwrap_or_else(|_| "[]".to_string()),
        None => existing.arguments,
    };

    let updated = db_update_media_launch(
        &state.db,
        &media_id,
        &launch_id,
        MediaLaunchUpdate {
            agent_id,
            name,
            launch_type,
            emulator_id,
            program,
            arguments,
            argument_mode,
            working_directory,
            game_location,
            is_default: payload.is_default.unwrap_or(existing.is_default),
            enabled: payload.enabled.unwrap_or(existing.enabled),
        },
    )
    .await?;

    Ok(Json(Launch::from(updated)))
}

#[utoipa::path(
    delete,
    path = "/media/{media_id}/launches/{launch_id}",
    params(
        ("media_id" = String, Path, description = "ID of the media"),
        ("launch_id" = String, Path, description = "ID of the launch profile")
    ),
    responses(
        (status = 204, description = "Delete a launch profile")
    )
)]
#[instrument(skip(state))]
pub async fn delete_media_launch(
    State(state): State<AppState>,
    Path((media_id, launch_id)): Path<(String, String)>,
) -> Result<StatusCode, ApiError> {
    let rows_affected = db_delete_media_launch(&state.db, &media_id, &launch_id).await?;
    if rows_affected == 0 {
        return Err(ApiError::not_found("Launch profile not found"));
    }
    Ok(StatusCode::NO_CONTENT)
}
