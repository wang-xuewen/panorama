use crate::sqlite_sample::sqlite_c::SqliteCrud;
use anyhow::Result;
use once_cell::sync::OnceCell;
use std::sync::{Arc, Mutex};

pub static GLOBAL_DB: OnceCell<Arc<Mutex<SqliteCrud>>> = OnceCell::new();
pub const SQLITE_DB_PATH: &str =
    "/Users/zongge/rust/panorama/panorama_s/src/sqlite_sample/sqlite_sample.db";
pub const LOG4RS_YAML_PATH: &str = "/Users/zongge/rust/panorama/panorama_s/log4rs.yaml";

pub fn init_global_db(db_path: &str) -> Result<()> {
    let db = SqliteCrud::new(db_path)?;
    GLOBAL_DB
        .set(Arc::new(Mutex::new(db)))
        .map_err(|_| anyhow::anyhow!("GLOBAL_DB already initialized"))?;
    Ok(())
}
pub fn get_global_db() -> Result<Arc<Mutex<SqliteCrud>>> {
    GLOBAL_DB
        .get()
        .map(Arc::clone)
        .ok_or_else(|| anyhow::anyhow!("GLOBAL_DB not initialized"))
}
