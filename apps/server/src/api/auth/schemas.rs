use itonda_domain::storefronts::models::StorefrontId;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct StorefrontAuthStatusResponse {
    pub storefront: StorefrontId,
    pub connected: bool,
    pub steam_id: Option<String>,
    pub account_name: Option<String>,
    pub avatar_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AuthUrlResponse {
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AuthActionResponse {
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SteamCallbackPayload {
    pub params: Vec<(String, String)>,
}
