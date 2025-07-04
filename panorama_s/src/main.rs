mod common;
mod sqlite_sample;
mod use_sqlite;
mod web_server;
use crate::common::global;
use crate::use_sqlite::use_sqlite;
use crate::web_server::web_server_main;
use anyhow::Result;

use log::{error, info};
use log4rs;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    // 初始化日志系统
    if let Err(e) = log4rs::init_file(
        global::LOG4RS_YAML_PATH,
        Default::default(),
    ) {
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
    if let Err(e) = web_server_main::run_server().await {
        error!("Server error: {}", e);
    }
}

fn init() -> Result<()> {
    global::init_global_db(global::SQLITE_DB_PATH)?;
    Ok(())
}
