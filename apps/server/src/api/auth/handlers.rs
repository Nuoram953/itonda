use std::sync::Arc;

use axum::{
    Json,
    extract::{Query, State},
    http::HeaderMap,
    response::{IntoResponse, Redirect, Response},
};
use itonda_domain::storefronts::{
    auth::{StorefrontAuthenticator, steam::SteamAuthenticator},
    models::StorefrontId,
    steam::{SteamStorefront, client::SteamClient},
};
use serde::Deserialize;
use tracing::instrument;

use crate::{
    api::{
        auth::schemas::{
            AuthActionResponse, AuthUrlResponse, SteamCallbackPayload,
            StorefrontAuthStatusResponse,
        },
        error::ApiError,
    },
    state::AppState,
};

#[derive(Debug, Deserialize)]
pub struct LoginQuery {
    pub redirect: Option<bool>,
    pub return_url: Option<String>,
}

#[utoipa::path(
    get,
    path = "/auth/steam/login",
    responses(
        (status = 307, description = "Redirects to Steam OpenID authentication"),
        (status = 200, body = AuthUrlResponse),
    )
)]
#[instrument(skip(_state, headers))]
pub async fn steam_login(
    State(_state): State<AppState>,
    headers: HeaderMap,
    Query(query): Query<LoginQuery>,
) -> Response {
    // Determine frontend origin from referer/origin or fallback to localhost
    let default_origin = headers
        .get("origin")
        .or_else(|| headers.get("referer"))
        .and_then(|h| h.to_str().ok())
        .and_then(|url_str| url::Url::parse(url_str).ok())
        .map(|u| u.origin().ascii_serialization())
        .unwrap_or_else(|| "http://localhost:5173".to_string());

    let return_to = query
        .return_url
        .unwrap_or_else(|| format!("{default_origin}/auth/callback/steam"));

    let realm = if let Ok(u) = url::Url::parse(&return_to) {
        format!("{}/", u.origin().ascii_serialization())
    } else {
        format!("{default_origin}/")
    };

    let authenticator = SteamAuthenticator::new();
    let auth_url = authenticator.generate_auth_url(&return_to, &realm);

    if query.redirect == Some(false) {
        Json(AuthUrlResponse { url: auth_url }).into_response()
    } else {
        Redirect::temporary(&auth_url).into_response()
    }
}

#[utoipa::path(
    post,
    path = "/auth/steam/callback",
    request_body = SteamCallbackPayload,
    responses(
        (status = 200, body = StorefrontAuthStatusResponse),
        (status = 400, description = "OpenID signature validation failed"),
    )
)]
#[instrument(skip(state))]
pub async fn steam_callback(
    State(state): State<AppState>,
    Json(payload): Json<SteamCallbackPayload>,
) -> Result<Json<StorefrontAuthStatusResponse>, ApiError> {
    let authenticator = SteamAuthenticator::new();
    let profile = authenticator.verify_callback(&payload.params).await?;
    let steam_id = profile.external_id;

    // Fetch player summary (persona name & avatar)
    let secrets = state.secrets.get().await;
    let steam_client = SteamClient::new(&secrets.storefronts.steam.api_key);
    let summary = steam_client
        .get_player_summary(&steam_id)
        .await
        .unwrap_or(None);

    let account_name = summary.as_ref().and_then(|s| s.personaname.clone());
    let avatar_url = summary
        .as_ref()
        .and_then(|s| s.avatarfull.clone().or_else(|| s.avatar.clone()));

    state
        .secrets
        .update(|s| {
            s.storefronts.steam.steam_id = steam_id.clone();
            s.storefronts.steam.account_name = account_name.clone();
            s.storefronts.steam.avatar_url = avatar_url.clone();
        })
        .await?;

    state.storefronts.register(Arc::new(SteamStorefront::new(
        secrets.storefronts.steam.api_key,
        steam_id.clone(),
    )));

    Ok(Json(StorefrontAuthStatusResponse {
        storefront: StorefrontId::Steam,
        connected: true,
        steam_id: Some(steam_id),
        account_name,
        avatar_url,
    }))
}

#[utoipa::path(
    get,
    path = "/auth/steam/status",
    responses(
        (status = 200, body = StorefrontAuthStatusResponse),
    )
)]
#[instrument(skip(state))]
pub async fn steam_status(
    State(state): State<AppState>,
) -> Result<Json<StorefrontAuthStatusResponse>, ApiError> {
    let secrets = state.secrets.get().await;
    let steam_id = secrets.storefronts.steam.steam_id;
    let connected = !steam_id.is_empty() && steam_id != "0";

    let (account_name, avatar_url) = if connected {
        if secrets.storefronts.steam.account_name.is_none()
            || secrets.storefronts.steam.avatar_url.is_none()
        {
            let steam_client = SteamClient::new(&secrets.storefronts.steam.api_key);
            if let Ok(Some(summary)) = steam_client.get_player_summary(&steam_id).await {
                let name = summary.personaname;
                let avatar = summary.avatarfull.or(summary.avatar);
                let _ = state
                    .secrets
                    .update(|s| {
                        s.storefronts.steam.account_name = name.clone();
                        s.storefronts.steam.avatar_url = avatar.clone();
                    })
                    .await;
                (name, avatar)
            } else {
                (
                    secrets.storefronts.steam.account_name,
                    secrets.storefronts.steam.avatar_url,
                )
            }
        } else {
            (
                secrets.storefronts.steam.account_name,
                secrets.storefronts.steam.avatar_url,
            )
        }
    } else {
        (None, None)
    };

    Ok(Json(StorefrontAuthStatusResponse {
        storefront: StorefrontId::Steam,
        connected,
        steam_id: if connected { Some(steam_id) } else { None },
        account_name,
        avatar_url,
    }))
}

#[utoipa::path(
    post,
    path = "/auth/steam/disconnect",
    responses(
        (status = 200, body = AuthActionResponse),
    )
)]
#[instrument(skip(state))]
pub async fn steam_disconnect(
    State(state): State<AppState>,
) -> Result<Json<AuthActionResponse>, ApiError> {
    state
        .secrets
        .update(|s| {
            s.storefronts.steam.steam_id = String::new();
            s.storefronts.steam.account_name = None;
            s.storefronts.steam.avatar_url = None;
        })
        .await?;

    state.storefronts.remove(StorefrontId::Steam);

    Ok(Json(AuthActionResponse {
        success: true,
        message: "Steam account disconnected successfully".into(),
    }))
}
