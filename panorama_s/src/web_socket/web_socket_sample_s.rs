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

// WebSocket 背压处理及Rust示例
use futures::{SinkExt, StreamExt};
use tokio::sync::mpsc;
use tokio_tungstenite::connect_async;
use tungstenite::protocol::Message;

#[tokio::main]
async fn main() {
    // 连接到WebSocket服务器
    let url = "wss://echo.websocket.org";
    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    println!("Connected to {}", url);

    // 分离读写部分
    let (mut write_half, mut read_half) = ws_stream.split();

    // 创建一个有界通道用于背压控制（缓冲区大小为10）
    let (sender, mut receiver) = mpsc::channel::<Message>(10);

    // 启动发送任务
    let send_task = tokio::spawn(async move {
        while let Some(message) = receiver.recv().await {
            // 尝试发送消息，如果遇到背压会在这里等待
            if let Err(e) = write_half.send(message).await {
                eprintln!("Send error: {}", e);
                break;
            }
        }
    });

    // 启动接收任务
    let recv_task = tokio::spawn(async move {
        while let Some(msg) = read_half.next().await {
            match msg {
                Ok(msg) => {
                    println!("Received: {}", msg);
                    // 模拟处理延迟
                    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                }
                Err(e) => {
                    eprintln!("Receive error: {}", e);
                    break;
                }
            }
        }
    });

    // 主线程模拟消息生产
    for i in 1..=20 {
        let message = Message::Text(format!("Message {}", i));
        println!("Trying to send: {}", message);

        // 使用try_send来检测背压，如果缓冲区满了就等待
        if let Err(e) = sender.try_send(message) {
            println!("Backpressure detected! Waiting... (attempt {})", i);
            // 等待一段时间再重试
            tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
            // 这里可以加入更复杂的背压处理逻辑
            continue;
        }

        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
    }

    // 关闭发送通道
    drop(sender);

    // 等待任务完成
    let _ = tokio::join!(send_task, recv_task);
}
