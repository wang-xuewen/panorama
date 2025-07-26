use std::result;

use anyhow::Result;
use futures::{future::ok, SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message, MaybeTlsStream, WebSocketStream};
use url::Url;
use log::{error, info};
use std::error::Error;
use tungstenite::error::{Error as WebSocketError, Result as WebSocketResult};

use crate::web_socket;

pub struct WebSocketClient {
    ws_stream:Option<WebSocketStream<MaybeTlsStream<TcpStream>>>
}

impl WebSocketClient {
    pub fn new() -> Self {
        info!("ws new.");
        WebSocketClient{ws_stream:None}
    }

    pub async fn connect(&mut self,url:&str) -> Result<(), Box<dyn Error>> {
        let url = Url::parse(url)?;
        let (ws_stream,_) = connect_async(url).await?;
        self.ws_stream= Some(ws_stream);
        info!("ws connect ok.");
        Ok(())
    }

    pub async fn close(&mut self) -> Result<(), Box<dyn Error>> {
        if let Some(stream) = &mut self.ws_stream {
            stream.close(None).await?;
            self.ws_stream = None;
        }
        Ok(())
    }

    /// 发送消息
    pub async fn send_message(&mut self, message: &str) -> Result<(), Box<dyn Error>> {
        if let Some(stream) = &mut self.ws_stream {
            stream
                .send(Message::Text(message.to_string()))
                .await?;
        } else {
            error!("Not connected to WebSocket server")
        }
         Ok(())
    }

    /// 接收消息
    pub async fn receive_message(&mut self) -> Result<Option<String>,Box<dyn Error>> {
        if let Some(stream) = &mut self.ws_stream {
            if let Some(msg) = stream.next().await {
                match msg? {
                    Message::Text(text) => Ok(Some(text)),
                    Message::Close(_) => {
                        // self.close().await?;
                        let result = self.close().await;
                        if let Err(e) = result {
                            // 处理错误
                            error!("Receive failed. {}", e)
                        }
                         Ok(None)
                    },
                    _ =>  {
                        info!("Received msg ");
                         Ok(None)
                    } 
                }
            } else {
                 Ok(None)
            }
        } else {
            error!("Not connected to WebSocket server");
             Err(Box::new(WebSocketError::ConnectionClosed))
        }
    }
}