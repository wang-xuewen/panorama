#![allow(unused)] // 全局屏蔽 unused 警告

mod common;
mod web_socket;

use crate::common::global;
// use crate::web_socket::use_ws_client;
use crate::web_socket::ws_client_1;
use anyhow::Result;
use log::{error, info};
use log4rs;
use rust_utils::graceful_shutdown;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    // 初始化日志系统
    if let Err(e) = log4rs::init_file(global::LOG4RS_YAML_PATH, Default::default()) {
        eprintln!("init log4rs Error: {}", e);
    }
    info!("init log4rs ok.");

    match init() {
        Ok(_) => info!("[init] ok."),
        Err(e) => error!("[init] failed: {}", e),
    }

    let _ = tokio::spawn(async {
        if let Err(e) = ws_client_1::ws_client_sample().await {
            eprintln!("ws_client_sample error: {}", e);
        }
    });

    // let _ = tokio::spawn(async {
    //     if let Err(e) = use_ws_client::use_ws().await {
    //         error!("ws_client_sample error: {}", e);
    //     }
    // });
   
   info!("wait to close.");

    match graceful_shutdown().await {
        Ok(()) => info!("Shutdown successful"),
        Err(e) => error!("Shutdown failed: {}", e),
    }
}

fn init() -> Result<()> {
    Ok(())
}
