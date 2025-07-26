use log::{error, info};
use crate::web_socket::ws_client;
use std::error::Error;
use anyhow::Result;

pub async fn use_ws()  -> Result<(), Box<dyn Error>> {
    
    let mut client = ws_client::WebSocketClient::new();
    
    // 连接服务器
    client.connect("ws://127.0.0.1:8080").await?;
    info!("Connected to WebSocket server");

    // 发送消息
    client.send_message("Hello, WebSocket!").await?;
    info!("Sent message");

    // 接收消息
    if let Some(response) = client.receive_message().await? {
        info!("Received: {}", response);
    }

    // 正常关闭
    client.close().await?;
    info!("Connection closed");

    Ok(())
}

