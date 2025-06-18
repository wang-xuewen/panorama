mod common;
mod sqlite_sample;
mod use_sample;
use crate::common::global;
use crate::use_sample::use_sqlite;
use anyhow::Result;

use log::{error, info};
use log4rs;

fn main() {
    println!("Hello, world!");

    // 初始化日志系统
    if let Err(e) = log4rs::init_file("log4rs.yaml", Default::default()) {
        eprintln!("init log4rs Error: {}", e);
    }
    info!("init log4rs ok.");

    match init() {
        Ok(_) => info!("init ok."),
        Err(e) => error!("init failed: {}", e),
    }
    match use_sqlite() {
        Ok(_) => info!("use_sqlite ok."),
        Err(e) => error!("use_sqlite failed: {}", e),
    }
}

fn init() -> Result<()> {
    global::init_global_db("")?;
    Ok(())
}
