use anyhow::Result;
use core::option::Option::None;
use futures_util::{stream::SplitSink, SinkExt, StreamExt};
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

    // 任务1：读取 WebSocket 消息（含超时）
    let read_task = tokio::spawn(async move {
        loop {
            match timeout(Duration::from_millis(READ_TIMEOUT_MS), ws_receiver.next()).await {
                Ok(Some(Ok(msg))) => {
                    match msg {
                        Message::Close(close_frame) => {
                            println!("收到关闭帧，准备关闭连接");
                            // 显式响应关闭帧
                            let _ = ws_sender.close().await;
                            break; // 退出循环以释放资源
                        }
                        _ => {
                            println!("收到消息: {:?}", msg);

                            match msg {
                                Message::Text(text) => {
                                    // 回显消息
                                    // let ret_msg = Message::Text(format!("{}_ret", text));
                                    let ret_msg = Message::Text(text + "_ret");
                                    if let Err(e) = ws_sender.send(ret_msg.clone()).await {
                                        println!("Error sending message: {}", e);
                                        break;
                                    }
                                    println!("已发送消息: {:?}", ret_msg);
                                }
                                _ => {
                                    println!("收到消息不是Text类型，不做处理: {:?}", msg);
                                }
                            }
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

    println!("连接已关闭");
}

pub async fn run_server() -> Result<(), Box<dyn Error>> {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;
    while let Ok((stream, _)) = listener.accept().await {
        let ws_stream = accept_async(stream).await?;
        tokio::spawn(handle_connection(ws_stream));
    }
    Ok(())
}
