use futures_util::{SinkExt, StreamExt};
use itonda_domain::protocol::ServerToAgentMessage;
use serde::Serialize;
use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, tungstenite};
use tracing::{debug, error, trace, warn};
use tungstenite::Message;

pub struct AgentConnection {
    socket: WebSocketStream<MaybeTlsStream<TcpStream>>,
}

impl AgentConnection {
    pub async fn connect(url: &str) -> anyhow::Result<Self> {
        debug!("Connecting WebSocket to {url}");
        let (socket, _) = tokio_tungstenite::connect_async(url).await.map_err(|err| {
            error!("WebSocket connection failed to {url}: {err}");
            err
        })?;

        debug!("WebSocket connection established to {url}");
        Ok(Self { socket })
    }

    pub async fn receive(&mut self) -> anyhow::Result<ServerToAgentMessage> {
        while let Some(message) = self.socket.next().await {
            let message = message.map_err(|err| {
                error!("WebSocket receive error: {err}");
                err
            })?;

            match message {
                Message::Text(text) => {
                    trace!("Received WebSocket raw message: {text}");
                    let command = serde_json::from_str::<ServerToAgentMessage>(&text).map_err(|err| {
                        error!("Failed to deserialize ServerToAgentMessage: {err}. Raw text: {text}");
                        err
                    })?;
                    return Ok(command);
                }
                Message::Ping(payload) => {
                    trace!("Received WS ping frame, replying with WS pong");
                    let _ = self.socket.send(Message::Pong(payload)).await;
                }
                Message::Pong(_) => {
                    trace!("Received WS pong frame");
                }
                Message::Close(frame) => {
                    warn!("Server closed WebSocket connection: {frame:?}");
                    return Err(anyhow::anyhow!("Connection closed by server"));
                }
                _ => {
                    trace!("Ignoring WS frame: {message:?}");
                }
            }
        }

        warn!("WebSocket stream ended");
        Err(anyhow::anyhow!("Connection closed"))
    }

    pub async fn send<T: Serialize>(&mut self, value: &T) -> anyhow::Result<()> {
        let json = serde_json::to_string(value).map_err(|err| {
            error!("Failed to serialize message: {err}");
            err
        })?;
        trace!("Sending WebSocket message: {json}");
        self.socket
            .send(Message::Text(json.into()))
            .await
            .map_err(|err| {
                error!("WebSocket send error: {err}");
                err
            })?;

        Ok(())
    }
}
