use futures_util::{SinkExt, StreamExt, stream::SplitSink};
use tokio::net::TcpStream;
use tokio_tungstenite::{accept_async, WebSocketStream, tungstenite::Message};
use tokio::time::{timeout, Duration};
use tokio::sync::mpsc;

// 设置读写超时（毫秒）
const READ_TIMEOUT_MS: u64 = 5000;
const WRITE_TIMEOUT_MS: u64 = 5000;

async fn handle_connection(ws_stream: WebSocketStream<TcpStream>) {
    // 拆分成读/写两端
    let (mut ws_sender, mut ws_receiver) = ws_stream.split();

    // 使用 channel 控制消息积压（缓冲区大小 100）
    let (tx, mut rx) = mpsc::channel::<Message>(100);

    // 任务1：读取 WebSocket 消息（含超时）
    let read_task = tokio::spawn(async move {
        loop {
            match timeout(Duration::from_millis(READ_TIMEOUT_MS), ws_receiver.next()).await {
                Ok(Some(Ok(msg))) => {
                    match msg {
                        Message::Close(_) => {
                            println!("收到关闭帧，准备关闭连接");
                            break; // 正常关闭
                        }
                        _ => {
                            println!("收到消息: {:?}", msg);
                            // 可以在这里处理业务逻辑
                        }
                    }
                }
                Ok(Some(Err(e))) => {
                    eprintln!("WebSocket 读错误: {}", e);
                    break;
                }
                Ok(None) => break, // 连接关闭
                Err(_) => {
                    eprintln!("读取超时，强制关闭连接");
                    break;
                }
            }
        }
    });

    // 任务2：写入 WebSocket 消息（含超时）
    let write_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if let Err(e) = timeout(
                Duration::from_millis(WRITE_TIMEOUT_MS),
                ws_sender.send(msg)
            ).await {
                eprintln!("发送超时或失败: {}", e);
                break;
            }
        }
    });

    // 等待任意一个任务结束
    tokio::select! {
        _ = read_task => (),
        _ = write_task => (),
    }

    println!("连接已关闭");
}

#[tokio::main]
async fn run_server() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();
    while let Ok((stream, _)) = listener.accept().await {
        let ws_stream = accept_async(stream).await.unwrap();
        tokio::spawn(handle_connection(ws_stream));
    }
}