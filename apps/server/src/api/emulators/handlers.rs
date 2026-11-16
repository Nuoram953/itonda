use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use itonda_database::emulators::{
    models::{EmulatorInsert, EmulatorUpdate},
    queries::{
        delete_emulator as db_delete_emulator, find_all_emulators, find_emulator_by_id,
        insert_emulator, update_emulator as db_update_emulator,
    },
};
use itonda_domain::emulators::{
    definitions::load_emulator_definitions,
    errors::EmulatorError,
    models::Emulator,
};
use tracing::instrument;

use crate::{
    api::{
        emulators::schemas::{
            CreateEmulatorPayload, GetEmulatorDefinitionsResponse, GetEmulatorsResponse,
            UpdateEmulatorPayload,
        },
        error::ApiError,
    },
    state::AppState,
};

#[utoipa::path(
    get,
    path = "/emulators/definitions",
    responses(
        (status = 200, body = GetEmulatorDefinitionsResponse, description = "List available emulator definition files from config")
    )
)]
#[instrument(skip(state))]
pub async fn get_emulator_definitions(
    State(state): State<AppState>,
) -> Result<Json<GetEmulatorDefinitionsResponse>, ApiError> {
    let items = load_emulator_definitions(&state.emulators_dir);
    Ok(Json(GetEmulatorDefinitionsResponse { items }))
}

#[utoipa::path(
    get,
    path = "/emulators",
    responses(
        (status = 200, body = GetEmulatorsResponse, description = "List all configured emulators")
    )
)]
#[instrument(skip(state))]
pub async fn get_emulators(
    State(state): State<AppState>,
) -> Result<Json<GetEmulatorsResponse>, ApiError> {
    let rows = find_all_emulators(&state.db).await?;
    let items = rows.into_iter().map(Emulator::from).collect();
    Ok(Json(GetEmulatorsResponse { items }))
}

#[utoipa::path(
    post,
    path = "/emulators",
    request_body = CreateEmulatorPayload,
    responses(
        (status = 201, body = Emulator, description = "Create an emulator configuration")
    )
)]
#[instrument(skip(state))]
pub async fn create_emulator(
    State(state): State<AppState>,
    Json(payload): Json<CreateEmulatorPayload>,
) -> Result<impl IntoResponse, ApiError> {
    let name = payload.name.trim().to_string();
    let program = payload.program.trim().to_string();

    if name.is_empty() {
        return Err(ApiError::validation("Emulator name cannot be empty"));
    }
    if program.is_empty() {
        return Err(ApiError::validation("Emulator program cannot be empty"));
    }

    let arguments = serde_json::to_string(&payload.arguments).unwrap_or_else(|_| "[]".to_string());
    let options = serde_json::to_string(&payload.options).unwrap_or_else(|_| "[]".to_string());
    let working_directory = payload
        .working_directory
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty());

    let row = insert_emulator(
        &state.db,
        EmulatorInsert {
            name,
            program,
            arguments,
            options,
            working_directory,
        },
    )
    .await
    .map_err(EmulatorError::from)?;

    Ok((StatusCode::CREATED, Json(Emulator::from(row))))
}

#[utoipa::path(
    patch,
    path = "/emulators/{emulator_id}",
    params(
        ("emulator_id" = String, Path, description = "ID of the emulator")
    ),
    request_body = UpdateEmulatorPayload,
    responses(
        (status = 200, body = Emulator, description = "Update an emulator configuration")
    )
)]
#[instrument(skip(state))]
pub async fn update_emulator(
    State(state): State<AppState>,
    Path(emulator_id): Path<String>,
    Json(payload): Json<UpdateEmulatorPayload>,
) -> Result<Json<Emulator>, ApiError> {
    let existing = find_emulator_by_id(&state.db, &emulator_id)
        .await
        .map_err(EmulatorError::from)?;

    let name = match payload.name {
        Some(n) => {
            let trimmed = n.trim().to_string();
            if trimmed.is_empty() {
                return Err(ApiError::validation("Emulator name cannot be empty"));
            }
            trimmed
        }
        None => existing.name,
    };

    let program = match payload.program {
        Some(p) => {
            let trimmed = p.trim().to_string();
            if trimmed.is_empty() {
                return Err(ApiError::validation("Emulator program cannot be empty"));
            }
            trimmed
        }
        None => existing.program,
    };

    let arguments = match payload.arguments {
        Some(args) => serde_json::to_string(&args).unwrap_or_else(|_| "[]".to_string()),
        None => existing.arguments,
    };

    let options = match payload.options {
        Some(opts) => serde_json::to_string(&opts).unwrap_or_else(|_| "[]".to_string()),
        None => existing.options,
    };

    let working_directory = match payload.working_directory {
        Some(wd) => wd.map(|s| s.trim().to_string()).filter(|s| !s.is_empty()),
        None => existing.working_directory,
    };

    let updated = db_update_emulator(
        &state.db,
        &emulator_id,
        EmulatorUpdate {
            name,
            program,
            arguments,
            options,
            working_directory,
        },
    )
    .await
    .map_err(EmulatorError::from)?;

    Ok(Json(Emulator::from(updated)))
}

#[utoipa::path(
    delete,
    path = "/emulators/{emulator_id}",
    params(
        ("emulator_id" = String, Path, description = "ID of the emulator")
    ),
    responses(
        (status = 204, description = "Delete an emulator configuration")
    )
)]
#[instrument(skip(state))]
pub async fn delete_emulator(
    State(state): State<AppState>,
    Path(emulator_id): Path<String>,
) -> Result<StatusCode, ApiError> {
    let rows_affected = db_delete_emulator(&state.db, &emulator_id).await?;
    if rows_affected == 0 {
        return Err(ApiError::not_found("Emulator not found"));
    }
    Ok(StatusCode::NO_CONTENT)
}
