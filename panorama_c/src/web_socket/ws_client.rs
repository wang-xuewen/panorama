// 客户端
use anyhow::Result;
use futures_util::{SinkExt, StreamExt};
use log::{error, info};
use std::error::Error;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
use url::Url;

pub async fn ws_client_sample() -> Result<(), Box<dyn Error>> {
    let url = Url::parse("ws://127.0.0.1:8080")?;

    info!("Connecting to {}", url);
    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    info!("WebSocket handshake has been successfully completed");

    let (mut write, mut read) = ws_stream.split();

    // 发送消息
    let message = Message::Text("Hello, WebSocket!".to_string());
    write.send(message).await.expect("Failed to send message");

    // 接收消息
    while let Some(msg) = read.next().await {
        match msg {
            Ok(msg) => {
                if msg.is_text() {
                    info!("Received message: {}", msg);
                } else if msg.is_close() {
                    info!("Server closed connection");
                    break;
                }
            }
            Err(e) => {
                error!("Error reading message: {}", e);
                break;
            }
        }
    }

    Ok(())
}
