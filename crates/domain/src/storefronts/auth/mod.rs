pub mod openid;
pub mod steam;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use utoipa::ToSchema;

use crate::storefronts::models::StorefrontId;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("OpenID validation failed: {0}")]
    OpenIdValidation(String),

    #[error("Invalid claimed ID in OpenID response: {0}")]
    InvalidClaimedId(String),

    #[error("HTTP request error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("Missing parameter: {0}")]
    MissingParameter(&'static str),

    #[error("Storefront error: {0}")]
    Storefront(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AuthProfile {
    pub storefront: StorefrontId,
    pub external_id: String,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
}

#[async_trait]
pub trait StorefrontAuthenticator: Send + Sync {
    fn storefront_id(&self) -> StorefrontId;
    fn generate_auth_url(&self, return_to: &str, realm: &str) -> String;
    async fn verify_callback(
        &self,
        params: &[(String, String)],
    ) -> Result<AuthProfile, AuthError>;
}
