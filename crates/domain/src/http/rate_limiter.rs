use async_trait::async_trait;
use reqwest::{Request, Response};
use reqwest_middleware::{Middleware, Next, Result};
use std::sync::Arc;
use tokio::sync::{Mutex, OwnedSemaphorePermit, Semaphore};
use tokio::time::{Duration, Instant, sleep};

#[derive(Debug)]
pub struct RateLimitPermit {
    _permit: OwnedSemaphorePermit,
}

#[derive(Debug)]
struct RateLimiterState {
    tokens: f64,
    max_tokens: f64,
    refill_rate: f64,
    last_refill: Instant,
}

#[derive(Debug, Clone)]
pub struct RateLimiter {
    semaphore: Arc<Semaphore>,
    state: Arc<Mutex<RateLimiterState>>,
}

impl RateLimiter {
    pub fn new(max_concurrency: usize, requests_per_second: f64) -> Self {
        let max_concurrency = max_concurrency.max(1);
        let requests_per_second = if requests_per_second <= 0.0 || requests_per_second.is_nan() {
            f64::INFINITY
        } else {
            requests_per_second
        };

        Self {
            semaphore: Arc::new(Semaphore::new(max_concurrency)),
            state: Arc::new(Mutex::new(RateLimiterState {
                tokens: requests_per_second,
                max_tokens: requests_per_second,
                refill_rate: requests_per_second,
                last_refill: Instant::now(),
            })),
        }
    }

    pub async fn acquire(&self) -> RateLimitPermit {
        let permit = self
            .semaphore
            .clone()
            .acquire_owned()
            .await
            .expect("semaphore closed");

        let delay = {
            let mut state = self.state.lock().await;
            let now = Instant::now();
            let elapsed = now.duration_since(state.last_refill).as_secs_f64();
            state.last_refill = now;

            if state.refill_rate.is_infinite() {
                Duration::ZERO
            } else {
                state.tokens = (state.tokens + elapsed * state.refill_rate).min(state.max_tokens);

                if state.tokens >= 1.0 {
                    state.tokens -= 1.0;
                    Duration::ZERO
                } else {
                    state.tokens -= 1.0;
                    let wait_secs = if state.refill_rate > 0.0 {
                        ((-state.tokens) / state.refill_rate).max(0.0)
                    } else {
                        0.0
                    };

                    if wait_secs.is_finite() && wait_secs > 0.0 {
                        Duration::from_secs_f64(wait_secs)
                    } else {
                        Duration::ZERO
                    }
                }
            }
        };

        if !delay.is_zero() {
            sleep(delay).await;
        }

        RateLimitPermit { _permit: permit }
    }

    pub fn available_concurrency_slots(&self) -> usize {
        self.semaphore.available_permits()
    }
}

pub struct RateLimitMiddleware {
    limiter: RateLimiter,
}

impl RateLimitMiddleware {
    pub fn new(limiter: RateLimiter) -> Self {
        Self { limiter }
    }
}

#[async_trait]
impl Middleware for RateLimitMiddleware {
    async fn handle(
        &self,
        req: Request,
        extensions: &mut http::Extensions,
        next: Next<'_>,
    ) -> Result<Response> {
        let _permit = self.limiter.acquire().await;
        next.run(req, extensions).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::http::create_rate_limited_http_client;
    use wiremock::{
        Mock, MockServer, ResponseTemplate,
        matchers::{method, path},
    };

    #[tokio::test]
    async fn test_rate_limiter_concurrency() {
        let limiter = RateLimiter::new(4, 100.0);
        assert_eq!(limiter.available_concurrency_slots(), 4);

        let mut permits = Vec::new();
        for _ in 0..4 {
            permits.push(limiter.acquire().await);
        }
        assert_eq!(limiter.available_concurrency_slots(), 0);

        permits.pop();
        assert_eq!(limiter.available_concurrency_slots(), 1);
    }

    #[tokio::test]
    async fn test_rate_limiter_pacing() {
        let limiter = RateLimiter::new(8, 4.0);

        let start = Instant::now();
        for _ in 0..4 {
            let _permit = limiter.acquire().await;
        }
        assert!(start.elapsed().as_millis() < 100);

        let _fifth = limiter.acquire().await;
        assert!(start.elapsed().as_millis() >= 200);
    }

    #[tokio::test]
    async fn test_rate_limit_middleware_throttles_http_requests() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/ping"))
            .respond_with(ResponseTemplate::new(200).set_body_string("pong"))
            .mount(&server)
            .await;

        let client = create_rate_limited_http_client(RateLimiter::new(4, 2.0));

        let start = Instant::now();
        client
            .get(format!("{}/ping", server.uri()))
            .send()
            .await
            .unwrap();
        client
            .get(format!("{}/ping", server.uri()))
            .send()
            .await
            .unwrap();
        assert!(start.elapsed().as_millis() < 150);

        client
            .get(format!("{}/ping", server.uri()))
            .send()
            .await
            .unwrap();
        assert!(start.elapsed().as_millis() >= 450);
    }
}
