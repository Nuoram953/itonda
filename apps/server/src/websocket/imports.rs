use axum::{
    extract::{State, WebSocketUpgrade},
    response::IntoResponse,
};

use axum::extract::ws::Message;

use crate::state::AppState;

pub async fn websocket(ws: WebSocketUpgrade, State(state): State<AppState>) -> impl IntoResponse {
    tracing::debug!("WebSocket connection requested");

    ws.on_upgrade(move |mut socket| async move {
        tracing::debug!("WebSocket connected");

        let mut events = state.events.subscribe();

        while let Ok(event) = events.recv().await {
            tracing::trace!(?event, "Sending websocket event");

            let json = match serde_json::to_string(&event) {
                Ok(json) => json,
                Err(err) => {
                    tracing::error!(?err, "Failed to serialize event");
                    continue;
                }
            };

            if socket.send(Message::Text(json.into())).await.is_err() {
                break;
            }
        }

        tracing::debug!("WebSocket disconnected");
    })
}
