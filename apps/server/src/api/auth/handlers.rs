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
    steam::SteamStorefront,
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

    let client_redirect = query
        .return_url
        .or_else(|| {
            headers
                .get("referer")
                .and_then(|r| r.to_str().ok())
                .map(|s| s.to_string())
        })
        .unwrap_or_else(|| "/settings".to_string());

    let callback_base = format!("{protocol}://{host}/api/v1/auth/steam/callback");
    let return_to = format!(
        "{}?client_redirect={}",
        callback_base,
        url::form_urlencoded::byte_serialize(client_redirect.as_bytes()).collect::<String>()
    );

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
    let client_redirect = params
        .iter()
        .find(|(k, _)| k == "client_redirect")
        .map(|(_, v)| v.clone())
        .unwrap_or_else(|| "/settings".to_string());

    let target_base = client_redirect.split('?').next().unwrap_or("/settings");

    let authenticator = SteamAuthenticator::new();

    match authenticator.verify_callback(&params).await {
        Ok(profile) => {
            let steam_id = profile.external_id;

            if let Err(e) = state
                .secrets
                .update(|s| {
                    s.storefronts.steam.steam_id = steam_id.clone();
                })
                .await
            {
                tracing::error!("Failed to update secrets with Steam ID: {e}");
                let html = format!(
                    r#"<!DOCTYPE html><html><head><meta charset="utf-8"><title>Steam Auth</title></head><body><script>
if (window.opener) {{
  window.opener.postMessage({{ type: 'STEAM_AUTH_ERROR', error: 'Failed to save Steam credentials' }}, '*');
  window.close();
}} else {{
  window.location.href = '{target_base}?auth=error&drawer=steam&error=Failed+to+save+credentials';
}}
</script></body></html>"#
                );
                return Html(html).into_response();
            }

            let secrets = state.secrets.get().await;
            state.storefronts.register(Arc::new(SteamStorefront::new(
                secrets.storefronts.steam.api_key,
                steam_id.clone(),
            )));

            let html = format!(
                r#"<!DOCTYPE html><html><head><meta charset="utf-8"><title>Steam Auth</title></head><body><script>
if (window.opener) {{
  window.opener.postMessage({{ type: 'STEAM_AUTH_SUCCESS', steamId: '{steam_id}' }}, '*');
  window.close();
}} else {{
  window.location.href = '{target_base}?auth=success&drawer=steam';
}}
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
  window.opener.postMessage({{ type: 'STEAM_AUTH_ERROR', error: '{error_msg}' }}, '*');
  window.close();
}} else {{
  window.location.href = '{target_base}?auth=error&drawer=steam';
}}
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

    Ok(Json(StorefrontAuthStatusResponse {
        storefront: StorefrontId::Steam,
        connected,
        steam_id: if connected { Some(steam_id) } else { None },
        account_name: None,
        avatar_url: None,
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
        })
        .await?;

    state.storefronts.remove(StorefrontId::Steam);

    Ok(Json(AuthActionResponse {
        success: true,
        message: "Steam account disconnected successfully".into(),
    }))
}
