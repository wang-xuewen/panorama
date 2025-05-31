use anyhow::{anyhow, Result};
use log::error;
use rusqlite::Connection;

pub struct SqliteCrud {
    pub conn: Option<Connection>, // 改为 Option 以便在 drop 时 take
}

impl SqliteCrud {
    /// 创建数据库连接
    // pub fn new(db_path: &str) -> Result<Self, rusqlite::Error> {
    //     let conn = Connection::open(db_path)?;
    //     Ok(Self { conn: Some(conn) })
    // }

    pub fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        Ok(Self { conn: Some(conn) })
    }

    pub fn get_conn(&self) -> Result<&Connection> {
        self.conn.as_ref().ok_or(anyhow!("Connection is None"))
    }
}

// 实现 Drop trait 来正确关闭连接
impl Drop for SqliteCrud {
    fn drop(&mut self) {
        if let Some(conn) = self.conn.take() {
            if let Err((_conn, err)) = conn.close() {
                error!("⚠️ Failed to close SQLite connection: {}", err);
            }
        }
    }
}
