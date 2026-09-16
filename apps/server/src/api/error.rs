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

    #[error("{0}")]
    NotFound(String),

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

    #[error("internal server error")]
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
    pub fn not_found(message: impl Into<String>) -> Self {
        Self::NotFound(message.into())
    }

    pub fn validation(message: impl Into<String>) -> Self {
        Self::Validation(message.into())
    }

    pub fn error_body(&self) -> ErrorResponse {
        ErrorResponse {
            code: self.code().into(),
            message: self.message(),
        }
    }

    fn status(&self) -> StatusCode {
        match self {
            Self::NotFound(_) => StatusCode::NOT_FOUND,

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
            Self::NotFound(_) => "NOT_FOUND",
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
            Self::NotFound(message) => message.clone(),
            Self::Validation(message) => message.clone(),
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
            LaunchError::NotFound => ApiError::not_found("Media launch not found"),

            LaunchError::NoAgentAvailable => ApiError::WorkerUnavailable,

            LaunchError::Database(err) => ApiError::Database(err),

            LaunchError::InvalidId => ApiError::Validation("Invalid launch id".into()),
        }
    }
}

impl From<MediaError> for ApiError {
    fn from(err: MediaError) -> Self {
        match err {
            MediaError::NotFound => ApiError::not_found("Media not found"),
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

impl From<itonda_domain::reviews::ReviewError> for ApiError {
    fn from(err: itonda_domain::reviews::ReviewError) -> Self {
        match err {
            itonda_domain::reviews::ReviewError::MediaNotFound(_) => {
                ApiError::not_found("Media not found")
            }
            itonda_domain::reviews::ReviewError::ThoughtNotFound(_) => {
                ApiError::not_found("Thought not found")
            }
            itonda_domain::reviews::ReviewError::InvalidVerdict(msg) => {
                ApiError::Validation(format!("Invalid verdict: {msg}"))
            }
            itonda_domain::reviews::ReviewError::Validation(msg) => ApiError::Validation(msg),
            itonda_domain::reviews::ReviewError::Database(err) => ApiError::Database(err),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not_found_status_code_and_body() {
        let err = ApiError::not_found("Custom resource missing");
        assert_eq!(err.status(), StatusCode::NOT_FOUND);
        let body = err.error_body();
        assert_eq!(body.code, "NOT_FOUND");
        assert_eq!(body.message, "Custom resource missing");
        assert_eq!(err.to_string(), "Custom resource missing");
    }

    #[test]
    fn test_validation_status_code_and_body() {
        let err = ApiError::validation("Field is required");
        assert_eq!(err.status(), StatusCode::UNPROCESSABLE_ENTITY);
        let body = err.error_body();
        assert_eq!(body.code, "VALIDATION_FAILED");
        assert_eq!(body.message, "Field is required");
    }

    #[test]
    fn test_from_media_error_not_found() {
        let err: ApiError = MediaError::NotFound.into();
        assert_eq!(err.status(), StatusCode::NOT_FOUND);
        let body = err.error_body();
        assert_eq!(body.code, "NOT_FOUND");
        assert_eq!(body.message, "Media not found");
    }

    #[test]
    fn test_from_launch_error_not_found() {
        let err: ApiError = LaunchError::NotFound.into();
        assert_eq!(err.status(), StatusCode::NOT_FOUND);
        let body = err.error_body();
        assert_eq!(body.code, "NOT_FOUND");
        assert_eq!(body.message, "Media launch not found");
    }
}
