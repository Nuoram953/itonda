use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum ApiError {
    InvalidPayload,
    MediaNotFound,
    CollectionNotFound,
    Validation(String),
    Database,
    WorkerUnavailable,
    Unauthorized,
    Forbidden,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
    pub code: String,
    pub message: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (self.status(), Json(self.response())).into_response()
    }
}

impl ApiError {
    pub fn response(&self) -> ErrorResponse {
        ErrorResponse {
            code: self.code().to_string(),
            message: self.message(),
        }
    }

    fn status(&self) -> StatusCode {
        match self {
            Self::MediaNotFound => StatusCode::NOT_FOUND,
            Self::CollectionNotFound => StatusCode::NOT_FOUND,

            Self::Validation(_) => StatusCode::UNPROCESSABLE_ENTITY,

            Self::Unauthorized => StatusCode::UNAUTHORIZED,
            Self::Forbidden => StatusCode::FORBIDDEN,

            Self::WorkerUnavailable => StatusCode::SERVICE_UNAVAILABLE,

            Self::Database => StatusCode::INTERNAL_SERVER_ERROR,

            Self::InvalidPayload => StatusCode::BAD_REQUEST,
        }
    }

    fn code(&self) -> &'static str {
        match self {
            Self::MediaNotFound => "MEDIA_NOT_FOUND",
            Self::CollectionNotFound => "COLLECTION_NOT_FOUND",

            Self::Validation(_) => "VALIDATION_FAILED",

            Self::Unauthorized => "UNAUTHORIZED",
            Self::Forbidden => "FORBIDDEN",

            Self::WorkerUnavailable => "WORKER_UNAVAILABLE",

            Self::Database => "DATABASE_ERROR",

            Self::InvalidPayload => "INVALID_PAYLOAD",
        }
    }

    fn message(&self) -> String {
        match self {
            Self::Validation(message) => message.clone(),

            Self::MediaNotFound => "Media not found".into(),

            Self::Database => "An unexpected error occurred.".into(),

            Self::WorkerUnavailable => "The import worker is currently unavailable.".into(),

            _ => "message missing".into(),
        }
    }
}
