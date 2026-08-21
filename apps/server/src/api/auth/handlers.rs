use std::sync::Arc;

use axum::{
    Json,
    extract::{Query, State},
    http::{HeaderMap, StatusCode},
    response::{Html, IntoResponse, Redirect, Response},
};
use itonda_domain::storefronts::{
    auth::{
        StorefrontAuthenticator,
        steam::SteamAuthenticator,
    },
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
        (status = 200, description = "Returns HTML completion script"),
        (status = 400, description = "Authentication validation failed"),
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

            // Update configuration in secrets.toml
            if let Err(e) = state
                .secrets
                .update(|s| {
                    s.storefronts.steam.steam_id = steam_id.clone();
                })
                .await
            {
                tracing::error!("Failed to update secrets with Steam ID: {e}");
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Html("<h2>Failed to save Steam credentials</h2>"),
                )
                    .into_response();
            }

            // Register updated provider in StorefrontRegistry
            let secrets = state.secrets.get().await;
            state.storefronts.register(Arc::new(SteamStorefront::new(
                secrets.storefronts.steam.api_key,
                steam_id.clone(),
            )));

            let html = format!(
                r#"<!DOCTYPE html>
<html>
<head>
  <meta charset="utf-8">
  <title>Steam Authentication</title>
  <style>
    body {{
      background-color: #0f172a;
      color: #f8fafc;
      font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
      display: flex;
      align-items: center;
      justify-content: center;
      height: 100vh;
      margin: 0;
    }}
    .card {{
      text-align: center;
      background: #1e293b;
      padding: 2rem;
      border-radius: 1rem;
      box-shadow: 0 10px 25px rgba(0,0,0,0.5);
      border: 1px solid rgba(255,255,255,0.1);
    }}
    h2 {{ margin-top: 0; color: #38bdf8; }}
  </style>
</head>
<body>
  <div class="card">
    <h2>Steam Connected!</h2>
    <p>SteamID: {steam_id}</p>
    <p>Closing window...</p>
  </div>
  <script>
    try {{
      if (window.opener) {{
        window.opener.postMessage({{
          type: 'STEAM_AUTH_SUCCESS',
          storefront: 'steam',
          steamId: '{steam_id}'
        }}, '*');
        setTimeout(() => window.close(), 600);
      }} else {{
        setTimeout(() => {{ window.location.href = '/settings?connected=steam'; }}, 1000);
      }}
    }} catch (e) {{
      window.location.href = '/settings?connected=steam';
    }}
  </script>
</body>
</html>"#
            );

            Html(html).into_response()
        }
        Err(e) => {
            tracing::warn!("Steam OpenID verification failed: {e}");
            let html = format!(
                r#"<!DOCTYPE html>
<html>
<head>
  <meta charset="utf-8">
  <title>Steam Authentication Failed</title>
  <style>
    body {{
      background-color: #0f172a;
      color: #f8fafc;
      font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
      display: flex;
      align-items: center;
      justify-content: center;
      height: 100vh;
      margin: 0;
    }}
    .card {{
      text-align: center;
      background: #1e293b;
      padding: 2rem;
      border-radius: 1rem;
      border: 1px solid #ef4444;
    }}
    h2 {{ margin-top: 0; color: #ef4444; }}
  </style>
</head>
<body>
  <div class="card">
    <h2>Authentication Failed</h2>
    <p>{e}</p>
  </div>
</body>
</html>"#
            );

            (StatusCode::BAD_REQUEST, Html(html)).into_response()
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
        steam_id: if connected {
            Some(steam_id)
        } else {
            None
        },
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
