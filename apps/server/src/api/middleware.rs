use axum::{
    body::{Body, to_bytes},
    extract::Request,
    middleware::Next,
    response::Response,
};
use std::time::Instant;

const MAX_ERROR_BODY_BYTES: usize = 64 * 1024;

pub async fn api_logging_middleware(req: Request, next: Next) -> Response {
    let start = Instant::now();
    let method = req.method().clone();
    let uri = req.uri().clone();
    let path = uri
        .path_and_query()
        .map(|pq| pq.as_str())
        .unwrap_or(uri.path())
        .to_string();

    let response = next.run(req).await;

    let status = response.status();
    let latency_ms = start.elapsed().as_millis();

    if status.is_client_error() || status.is_server_error() {
        let (parts, body) = response.into_parts();
        let bytes = match to_bytes(body, MAX_ERROR_BODY_BYTES).await {
            Ok(b) => b,
            Err(err) => {
                tracing::error!(
                    method = %method,
                    path = %path,
                    status = %status.as_u16(),
                    latency_ms = %latency_ms,
                    error = %err,
                    "Failed to buffer error response body"
                );
                return Response::from_parts(parts, Body::empty());
            }
        };

        let response_body = String::from_utf8_lossy(&bytes);

        tracing::error!(
            method = %method,
            path = %path,
            status = %status.as_u16(),
            latency_ms = %latency_ms,
            response = %response_body,
        );

        Response::from_parts(parts, Body::from(bytes))
    } else {
        tracing::info!(
            method = %method,
            path = %path,
            status = %status.as_u16(),
            latency_ms = %latency_ms
        );

        response
    }
}
