use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(Debug)]
pub enum ApiError {
    Database,
    InvalidRequest,
    WorkerUnavailable,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            ApiError::Database => (StatusCode::INTERNAL_SERVER_ERROR, "database error"),

            ApiError::InvalidRequest => (StatusCode::BAD_REQUEST, "invalid request"),

            ApiError::WorkerUnavailable => (StatusCode::SERVICE_UNAVAILABLE, "worker unavailable"),
        };

        (status, message).into_response()
    }
}
