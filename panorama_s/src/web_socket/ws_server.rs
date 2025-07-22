use anyhow::Result;
use core::option::Option::None;
use futures_util::{stream::SplitSink, SinkExt, StreamExt};
use log::{error, info};
use std::error::Error;
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio::time::{timeout, Duration};
use tokio_tungstenite::{accept_async, tungstenite::Message, WebSocketStream};

// 设置读写超时（毫秒）
const READ_TIMEOUT_MS: u64 = 5000;
const WRITE_TIMEOUT_MS: u64 = 5000;

async fn handle_connection(ws_stream: WebSocketStream<TcpStream>) {
    // 拆分成读/写两端
    let (mut ws_sender, mut ws_receiver) = ws_stream.split();

    // 持续监听接收消息
    while let Some(result) = ws_receiver.next().await {
        match result {
            Ok(Message::Text(text)) => {
                let ret_msg = Message::Text(format!("{}_ret", text));
                if ws_sender.send(ret_msg).await.is_err() {
                    break;
                }
            }
            Ok(Message::Binary(bin)) => {
                info!("Received binary (len={})", bin.len())
            }
            Ok(Message::Ping(ping)) => {
                let _ = ws_sender.send(Message::Pong(ping)).await;
            }
            Ok(Message::Close(_)) => break,
            Err(e) => {
                error!("接收错误: {}", e);
                break;
            }
            _ => {}
        }
    }
    // let _ = ws_sender.close().await; // 确保连接关闭
    match ws_sender.close().await {
        Ok(_) => println!("ws sender成功关闭"),
        Err(e) => error!("ws sender关闭失败：{}", e),
    }
}

pub async fn run_server() -> Result<(), Box<dyn Error>> {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;
    while let Ok((stream, _)) = listener.accept().await {
        let ws_stream = accept_async(stream).await?;
        tokio::spawn(handle_connection(ws_stream));
    }
    Ok(())
}
