use futures_util::{SinkExt, StreamExt};
use itonda_domain::protocol::server::ServerMessage;
use serde::Serialize;
use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, tungstenite};
use tungstenite::Message;

pub struct AgentConnection {
    socket: WebSocketStream<MaybeTlsStream<TcpStream>>,
}

impl AgentConnection {
    pub async fn connect(url: String) -> anyhow::Result<Self> {
        let (socket, _) = tokio_tungstenite::connect_async(url).await?;

        Ok(Self { socket })
    }

    pub async fn receive(&mut self) -> anyhow::Result<ServerMessage> {
        let message = self
            .socket
            .next()
            .await
            .ok_or_else(|| anyhow::anyhow!("Connection closed"))??;

        match message {
            Message::Text(text) => {
                let command = serde_json::from_str::<ServerMessage>(&text)?;
                Ok(command)
            }

            _ => Err(anyhow::anyhow!("Unexpected websocket message")),
        }
    }
    pub async fn send<T: Serialize>(&mut self, value: &T) -> anyhow::Result<()> {
        let json = serde_json::to_string(value)?;
        self.socket.send(Message::Text(json.into())).await?;

        Ok(())
    }
}
