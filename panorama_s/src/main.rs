#![allow(unused)] // 全局屏蔽 unused 警告
mod common;
mod rust_lang;
mod sqlite_sample;
mod use_sqlite;
mod web_server;
mod web_socket;

use crate::common::global;
use crate::rust_lang::trait_sample;
use crate::use_sqlite::use_sqlite;
use crate::web_server::web_server_main;
use crate::web_socket::ws_server;
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
    match use_sqlite() {
        Ok(_) => info!("[sqlite] use_sqlite ok."),
        Err(e) => error!("[sqlite] use_sqlite failed: {}", e),
    }

    // 启动 Web 服务器（后台运行）
    let _ = tokio::spawn(async {
        if let Err(e) = web_server_main::run_server().await {
            error!("web_server_main error: {}", e);
        }
    });

    let _ = tokio::spawn(async {
        if let Err(e) = ws_server::run_server().await {
            error!("web_server error: {}", e);
        }
    });

    trait_sample::use_trait();

    match graceful_shutdown().await {
        Ok(()) => info!("Shutdown successful"),
        Err(e) => error!("Shutdown failed: {}", e),
    }
}

fn init() -> Result<()> {
    global::init_global_db(global::SQLITE_DB_PATH)?;
    Ok(())
}
