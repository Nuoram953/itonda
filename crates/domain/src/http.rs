use async_trait::async_trait;
use reqwest::Client;
use reqwest::{Request, Response};
use reqwest_middleware::Result;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_middleware::{Middleware, Next};
use reqwest_tracing::TracingMiddleware;
use std::time::Instant;
use tracing::{error, info};

pub fn create_http_client() -> ClientWithMiddleware {
    let client = Client::builder()
        .build()
        .expect("failed to create http client");

    ClientBuilder::new(client)
        .with(LoggingMiddleware)
        .with(TracingMiddleware::default())
        .build()
}

pub struct LoggingMiddleware;

#[async_trait]
impl Middleware for LoggingMiddleware {
    async fn handle(
        &self,
        req: Request,
        extensions: &mut http::Extensions,
        next: Next<'_>,
    ) -> Result<Response> {
        let start = Instant::now();

        info!(
            method = %req.method(),
            url = %sanitize_url(req.url()),
            "HTTP request"
        );
        let result = next.run(req, extensions).await;

        match &result {
            Ok(response) => {
                info!(
                    status = %response.status(),
                    elapsed_ms = start.elapsed().as_millis(),
                    "HTTP response"
                );
            }
            Err(error) => {
                error!(
                    error = %error,
                    elapsed_ms = start.elapsed().as_millis(),
                    "HTTP request failed"
                );
            }
        }

        result
    }
}

fn sanitize_url(url: &reqwest::Url) -> String {
    let mut sanitized = url.clone();

    let query = url
        .query_pairs()
        .map(|(key, value)| {
            let value = match key.as_ref() {
                "key" | "api_key" | "token" => "***",
                _ => value.as_ref(),
            };

            format!("{key}={value}")
        })
        .collect::<Vec<_>>()
        .join("&");

    sanitized.set_query(Some(&query));

    sanitized.to_string()
}
