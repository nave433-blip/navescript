use tokio_tungstenite::{connect_async, tungstenite::protocol::Message, WebSocketStream};
use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_tungstenite::MaybeTlsStream;

pub struct WebSocketHost {
    stream: Option<WebSocketStream<MaybeTlsStream<TcpStream>>>,
}

impl WebSocketHost {
    pub async fn new(url: &str) -> Result<Self, String> {
        let (stream, _) = connect_async(url).await.map_err(|e| e.to_string())?;
        Ok(WebSocketHost { stream: Some(stream) })
    }

    pub async fn send(&mut self, message: String) -> Result<(), String> {
        if let Some(stream) = &mut self.stream {
            stream.send(Message::Text(message.into())).await.map_err(|e| e.to_string())
        } else {
            Err("Socket not open".to_string())
        }
    }

    pub async fn receive(&mut self) -> Result<String, String> {
        if let Some(stream) = &mut self.stream {
            let msg = stream.next().await.ok_or("Stream closed")?.map_err(|e| e.to_string())?;
            match msg {
                Message::Text(t) => Ok(t.to_string()),
                _ => Err("Received non-text message".to_string()),
            }
        } else {
            Err("Socket not open".to_string())
        }
    }

    pub async fn close(&mut self) -> Result<(), String> {
        if let Some(stream) = &mut self.stream {
            stream.close(None).await.map_err(|e| e.to_string())
        } else {
            Err("Socket already closed".to_string())
        }
    }
}
