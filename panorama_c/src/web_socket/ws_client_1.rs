// 客户端
use anyhow::Result;
use futures_util::{SinkExt, StreamExt};
use log::{error, info};
use std::error::Error;
use tokio::time::{interval, Duration};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
use url::Url;

pub async fn ws_client_sample() -> Result<(), Box<dyn Error>> {
    let url = Url::parse("ws://127.0.0.1:8080")?;

    info!("Connecting to {}", url);
    let (ws_stream, _) = connect_async(url).await?;
    info!("WebSocket handshake has been successfully completed");

    let (mut write, mut read) = ws_stream.split();

    // 创建心跳定时器（每30秒发送Ping）
    let mut ping_interval = interval(Duration::from_secs(5));

    // 使用有界通道处理背压
    let (tx, mut rx) = tokio::sync::mpsc::channel::<Message>(32);

    // 发送任务
    let send_task = tokio::spawn(async move {
        // 发送初始消息
        if let Err(e) = write.send(Message::Text("xxx".into())).await {
            error!("Failed to send initial message: {}", e);
            return;
        }

        loop {
            tokio::select! {
                // 处理来自通道的消息
                Some(msg) = rx.recv() => {
                    if let Err(e) = write.send(msg).await {
                        error!("Failed to send message: {}", e);
                        break;
                    }
                },
                // 定时发送Ping
                _ = ping_interval.tick() => {
                    if let Err(e) = write.send(Message::Ping(vec![])).await {
                        error!("Failed to send ping: {}", e);
                        break;
                    }
                    info!("send ping")
                },
            }
        }

        // 尝试正常关闭连接
        let _ = write.close().await;
    });

    // 接收任务
    let recv_task = tokio::spawn(async move {
        while let Some(msg) = read.next().await {
            match msg {
                Ok(Message::Pong(_)) => info!("Received pong"),
                Ok(Message::Close(_)) => break,
                Ok(other) => info!("Received msg: {:?}", other),
                Err(e) => {
                    error!("Receive error: {}", e);
                    break;
                }
            }
        }
    });

    // 在这里可以通过tx发送更多消息
    // 例如：tx.send(Message::Text("new message".into())).await?;

    // 等待任务完成
    tokio::select! {
        res = send_task => res?,
        res = recv_task => res?,
    }


    Ok(())
}
