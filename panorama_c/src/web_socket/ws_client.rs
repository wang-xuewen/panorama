// 客户端
use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::connect_async;
use tungstenite::protocol::Message;
use url::Url;

#[tokio::main]
async fn ws_client_sample() {
    let url = Url::parse("ws://127.0.0.1:8080").unwrap();

    println!("Connecting to {}", url);
    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    println!("WebSocket handshake has been successfully completed");

    let (mut write, mut read) = ws_stream.split();

    // 发送消息
    let message = Message::Text("Hello, WebSocket!".to_string());
    write.send(message).await.expect("Failed to send message");

    // 接收消息
    while let Some(msg) = read.next().await {
        match msg {
            Ok(msg) => {
                if msg.is_text() {
                    println!("Received message: {}", msg);
                } else if msg.is_close() {
                    println!("Server closed connection");
                    break;
                }
            }
            Err(e) => {
                println!("Error reading message: {}", e);
                break;
            }
        }
    }
}
