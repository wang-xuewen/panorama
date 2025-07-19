mod common;

use anyhow::Result;
use log::{error, info};
use log4rs;
use crate::common::global;
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

    match graceful_shutdown().await {
        Ok(()) => println!("Shutdown successful"),
        Err(e) => eprintln!("Shutdown failed: {}", e),
    }
   
}

fn init() -> Result<()> {
    Ok(())
}