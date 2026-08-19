use axum::{Json, extract::State};
use tracing::instrument;

use crate::{
    api::error::ApiError,
    config::{CombinedConfig, PatchConfigPayload},
    state::AppState,
};

#[utoipa::path(
    get,
    path = "/config",
    responses(
        (
            status = 200,
            body = CombinedConfig,
            description = "Get entire combined configuration"
        )
    )
)]
#[instrument(skip(state))]
pub async fn get_config(State(state): State<AppState>) -> Result<Json<CombinedConfig>, ApiError> {
    let config = CombinedConfig::from_state(&state).await;
    Ok(Json(config))
}

#[utoipa::path(
    patch,
    path = "/config",
    request_body = PatchConfigPayload,
    responses(
        (
            status = 200,
            body = CombinedConfig,
            description = "Update configuration partially and return updated combined configuration"
        )
    )
)]
#[instrument(skip(state))]
pub async fn update_config(
    State(state): State<AppState>,
    Json(payload): Json<PatchConfigPayload>,
) -> Result<Json<CombinedConfig>, ApiError> {
    if let Some(settings_patch) = payload.settings {
        state
            .settings
            .update(|settings| settings.apply_patch(settings_patch))
            .await?;
    }

    if let Some(secrets_patch) = payload.secrets {
        state
            .secrets
            .update(|secrets| secrets.apply_patch(secrets_patch))
            .await?;
    }

    if let Some(app_patch) = payload.app {
        state
            .config
            .update(|config| config.apply_patch(app_patch))
            .await?;
    }

    let config = CombinedConfig::from_state(&state).await;
    Ok(Json(config))
}
