use std::sync::Arc;

use axum::{
    Json,
    extract::{Query, State},
    http::HeaderMap,
    response::{Html, IntoResponse, Redirect, Response},
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
        auth::schemas::{AuthActionResponse, AuthUrlResponse, StorefrontAuthStatusResponse},
        error::ApiError,
    },
    state::AppState,
};

#[derive(Debug, Deserialize)]
pub struct LoginQuery {
    pub redirect: Option<bool>,
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
    let host = headers
        .get("host")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("localhost:3005");

    let protocol = if headers.contains_key("x-forwarded-proto") {
        headers
            .get("x-forwarded-proto")
            .and_then(|p| p.to_str().ok())
            .unwrap_or("http")
    } else {
        "http"
    };

    let realm = format!("{protocol}://{host}/");
    let return_to = format!("{protocol}://{host}/api/v1/auth/steam/callback");

    let authenticator = SteamAuthenticator::new();
    let auth_url = authenticator.generate_auth_url(&return_to, &realm);

    if query.redirect == Some(false) {
        Json(AuthUrlResponse { url: auth_url }).into_response()
    } else {
        Redirect::temporary(&auth_url).into_response()
    }
}

#[utoipa::path(
    get,
    path = "/auth/steam/callback",
    responses(
        (status = 200, description = "Sends message to opener and closes popup"),
    )
)]
#[instrument(skip(state))]
pub async fn steam_callback(
    State(state): State<AppState>,
    Query(params): Query<Vec<(String, String)>>,
) -> Response {
    let authenticator = SteamAuthenticator::new();

    match authenticator.verify_callback(&params).await {
        Ok(profile) => {
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

            if let Err(e) = state
                .secrets
                .update(|s| {
                    s.storefronts.steam.steam_id = steam_id.clone();
                    s.storefronts.steam.account_name = account_name.clone();
                    s.storefronts.steam.avatar_url = avatar_url.clone();
                })
                .await
            {
                tracing::error!("Failed to update secrets with Steam ID: {e}");
                let html = r#"<!DOCTYPE html><html><head><meta charset="utf-8"><title>Steam Auth</title></head><body><script>
if (window.opener) {
  window.opener.postMessage({ type: 'STEAM_AUTH_ERROR', error: 'Failed to save Steam credentials' }, '*');
}
window.close();
</script></body></html>"#;
                return Html(html).into_response();
            }

            state.storefronts.register(Arc::new(SteamStorefront::new(
                secrets.storefronts.steam.api_key,
                steam_id.clone(),
            )));

            let js_name = account_name.as_deref().unwrap_or("").replace('\'', "\\'");
            let js_avatar = avatar_url.as_deref().unwrap_or("").replace('\'', "\\'");

            let html = format!(
                r#"<!DOCTYPE html><html><head><meta charset="utf-8"><title>Steam Auth</title></head><body><script>
if (window.opener) {{
  window.opener.postMessage({{
    type: 'STEAM_AUTH_SUCCESS',
    steamId: '{steam_id}',
    accountName: '{js_name}',
    avatarUrl: '{js_avatar}'
  }}, '*');
}}
window.close();
</script></body></html>"#
            );

            Html(html).into_response()
        }
        Err(e) => {
            tracing::warn!("Steam OpenID verification failed: {e}");
            let error_msg = e.to_string().replace('\'', "\\'");
            let html = format!(
                r#"<!DOCTYPE html><html><head><meta charset="utf-8"><title>Steam Auth</title></head><body><script>
if (window.opener) {{
  window.opener.postMessage({{
    type: 'STEAM_AUTH_ERROR',
    error: '{error_msg}'
  }}, '*');
}}
window.close();
</script></body></html>"#
            );

            Html(html).into_response()
        }
    }
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
