// 客户端
use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::connect_async;
use tungstenite::protocol::Message;
use url::Url;

#[tokio::main]
async fn main() {
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



use futures::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message, MaybeTlsStream, WebSocketStream};
use url::Url;

pub struct WebSocketClient {
    ws_stream: Option<WebSocketStream<MaybeTlsStream<TcpStream>>>,
}

impl WebSocketClient {
    /// 创建新的 WebSocket 客户端
    pub fn new() -> Self {
        WebSocketClient { ws_stream: None }
    }

    /// 连接到 WebSocket 服务器
    pub async fn connect(&mut self, url: &str) -> Result<(), Box<dyn std::error::Error>> {
        let url = Url::parse(url)?;
        let (ws_stream, _) = connect_async(url).await?;
        self.ws_stream = Some(ws_stream);
        Ok(())
    }

    /// 发送消息
    pub async fn send_message(&mut self, message: &str) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(stream) = &mut self.ws_stream {
            stream
                .send(Message::Text(message.to_string()))
                .await?;
            Ok(())
        } else {
            Err("Not connected to WebSocket server".into())
        }
    }

    /// 接收消息
    pub async fn receive_message(&mut self) -> Result<Option<String>, Box<dyn std::error::Error>> {
        if let Some(stream) = &mut self.ws_stream {
            if let Some(msg) = stream.next().await {
                match msg? {
                    Message::Text(text) => Ok(Some(text)),
                    Message::Close(_) => {
                        self.close().await?;
                        Ok(None)
                    }
                    _ => Ok(None),
                }
            } else {
                Ok(None)
            }
        } else {
            Err("Not connected to WebSocket server".into())
        }
    }

    /// 正常关闭连接
    pub async fn close(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(stream) = &mut self.ws_stream {
            stream.close(None).await?;
            self.ws_stream = None;
        }
        Ok(())
    }
}