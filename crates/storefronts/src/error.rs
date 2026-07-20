use thiserror::Error;

#[derive(Debug, Error)]
pub enum StorefrontError {
    #[error("middleware http error: {0}")]
    Middleware(#[from] reqwest_middleware::Error),

    #[error("http request failed")]
    Http(#[from] reqwest::Error),

    #[error("authentication failed")]
    Authentication,

    #[error("access denied")]
    Forbidden,

    #[error("rate limited")]
    RateLimited,

    #[error("resource not found")]
    NotFound,

    #[error("invalid response from storefront")]
    InvalidResponse,

    #[error("storefront is unavailable")]
    Unavailable,

    #[error("{0}")]
    Other(String),
}
