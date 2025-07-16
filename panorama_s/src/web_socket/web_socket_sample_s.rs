// 服务端
use futures_util::{SinkExt, StreamExt};
use tokio::net::{TcpListener, TcpStream};
use tungstenite::protocol::Message;

async fn handle_connection(stream: TcpStream) {
    let addr = stream
        .peer_addr()
        .expect("connected streams should have a peer address");
    println!("New connection from: {}", addr);

    // 使用 tokio-tungstenite 创建 WebSocket
    let ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .expect("Error during the websocket handshake occurred");
    println!("WebSocket connection established: {}", addr);

    let (mut write, mut read) = ws_stream.split();

    // 处理接收到的消息
    while let Some(msg) = read.next().await {
        match msg {
            Ok(msg) => {
                if msg.is_text() || msg.is_binary() {
                    println!("Received a message from {}: {}", addr, msg);

                    // 回显消息
                    if let Err(e) = write.send(msg).await {
                        println!("Error sending message: {}", e);
                        break;
                    }
                } else if msg.is_close() {
                    println!("Client {} disconnected", addr);
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

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080")
        .await
        .expect("Failed to bind");
    println!("WebSocket server listening on ws://127.0.0.1:8080");

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(handle_connection(stream));
    }
}
