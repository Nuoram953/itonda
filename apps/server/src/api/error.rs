use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use itonda_domain::{agents::errors::AgentsError, launch::LaunchError, media::errors::MediaError};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("invalid payload")]
    InvalidPayload,

    #[error("media not found")]
    MediaNotFound,

    #[error("asset not found")]
    AssetNotFound,

    #[error("collection not found")]
    CollectionNotFound,

    #[error("media launch not found")]
    LaunchNotFound,

    #[error("{0}")]
    Validation(String),

    #[error("database error")]
    Database(#[from] itonda_database::error::DatabaseError),

    #[error("worker unavailable")]
    WorkerUnavailable,

    #[error("unauthorized")]
    Unauthorized,

    #[error("forbidden")]
    Forbidden,

    #[error("interal server error")]
    InternalServer,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
    pub code: String,
    pub message: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = self.status();

        (status, Json(self.error_body())).into_response()
    }
}

impl ApiError {
    pub fn error_body(&self) -> ErrorResponse {
        ErrorResponse {
            code: self.code().into(),
            message: self.message(),
        }
    }

    fn status(&self) -> StatusCode {
        match self {
            Self::MediaNotFound
            | Self::CollectionNotFound
            | Self::LaunchNotFound
            | Self::AssetNotFound => StatusCode::NOT_FOUND,

            Self::Validation(_) => StatusCode::UNPROCESSABLE_ENTITY,

            Self::Unauthorized => StatusCode::UNAUTHORIZED,

            Self::Forbidden => StatusCode::FORBIDDEN,

            Self::WorkerUnavailable => StatusCode::SERVICE_UNAVAILABLE,

            Self::InvalidPayload => StatusCode::BAD_REQUEST,

            Self::Database(_) | Self::InternalServer => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn code(&self) -> &'static str {
        match self {
            Self::MediaNotFound => "MEDIA_NOT_FOUND",
            Self::AssetNotFound => "ASSET_NOT_FOUND",
            Self::CollectionNotFound => "COLLECTION_NOT_FOUND",
            Self::LaunchNotFound => "LAUNCH_NOT_FOUND",
            Self::Validation(_) => "VALIDATION_FAILED",
            Self::Database(_) => "DATABASE_ERROR",
            Self::WorkerUnavailable => "WORKER_UNAVAILABLE",
            Self::Unauthorized => "UNAUTHORIZED",
            Self::Forbidden => "FORBIDDEN",
            Self::InvalidPayload => "INVALID_PAYLOAD",
            Self::InternalServer => "INTERNAL_SERVER_ERROR",
        }
    }

    fn message(&self) -> String {
        match self {
            Self::Validation(message) => message.clone(),
            Self::MediaNotFound => "Media not found".into(),
            Self::AssetNotFound => "Asset not found".into(),
            Self::CollectionNotFound => "Collection not found".into(),
            Self::LaunchNotFound => "Media launch not found".into(),
            Self::Database(_) => "An unexpected error occurred.".into(),
            Self::WorkerUnavailable => "No agent is currently available.".into(),
            Self::Unauthorized => "Unauthorized".into(),
            Self::Forbidden => "Forbidden".into(),
            Self::InvalidPayload => "Invalid payload".into(),
            Self::InternalServer => "Internal Server Error".into(),
        }
    }
}

impl From<LaunchError> for ApiError {
    fn from(err: LaunchError) -> Self {
        match err {
            LaunchError::NotFound => ApiError::LaunchNotFound,

            LaunchError::NoAgentAvailable => ApiError::WorkerUnavailable,

            LaunchError::Database(err) => ApiError::Database(err),

            LaunchError::InvalidId => ApiError::Validation("Invalid launch id".into()),
        }
    }
}

impl From<MediaError> for ApiError {
    fn from(err: MediaError) -> Self {
        match err {
            MediaError::NotFound => ApiError::MediaNotFound,
            MediaError::Database(err) => ApiError::Database(err),
            _ => ApiError::InvalidPayload,
        }
    }
}

impl From<AgentsError> for ApiError {
    fn from(err: AgentsError) -> Self {
        match err {
            AgentsError::Database(err) => ApiError::Database(err),
            AgentsError::NotConnected(msg) => {
                ApiError::Validation(format!("Agent not connected: {msg}"))
            }
            AgentsError::SendFailed(_) => ApiError::InternalServer,
        }
    }
}

impl From<itonda_domain::store::error::StoreError> for ApiError {
    fn from(_err: itonda_domain::store::error::StoreError) -> Self {
        ApiError::InternalServer
    }
}

impl From<itonda_domain::storefronts::auth::AuthError> for ApiError {
    fn from(err: itonda_domain::storefronts::auth::AuthError) -> Self {
        ApiError::Validation(err.to_string())
    }
}
