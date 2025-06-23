use crate::common::global;
use anyhow::{bail, Ok, Result};
use log::{error, info, warn};

pub fn use_sqlite() -> Result<()> {
    create_table()?;

    insert_data("aaa", "aaa_value")?;
    let result = query_data("aaa")?;
    info!("query_data result:{}", result);
    Ok(())
}

fn create_table() -> Result<()> {
    let db = global::get_global_db()?;
    let mut db_obj = db.lock().unwrap_or_else(|poisoned| {
        warn!("⚠️Mutex锁中毒，强制恢复访问");
        poisoned.into_inner() // 从中毒状态恢复数据访问
    });

    // 可以使用 CREATE TABLE IF NOT EXISTS users ？

    if let Some(conn) = db_obj.conn.as_mut() {
        let mut stmt = conn.prepare(
            "SELECT count(*) FROM sqlite_master
            WHERE type='table' AND name='table_test';",
        )?;
        let count: i64 = stmt.query_row([], |row| row.get(0))?;

        if count == 0 {
            conn.execute(
                "CREATE TABLE table_test (
                        id INTEGER PRIMARY KEY AUTOINCREMENT,
                        key TEXT NOT NULL,
                        value TEXT NULL
                    )",
                [],
            )?;
        }

        Ok(())
    } else {
        bail!("Connection is None")
    }
}

// 写入数据
fn insert_data(key: &str, value: &str) -> Result<()> {
    let db = global::get_global_db()?;
    let mut db_obj = db.lock().unwrap_or_else(|poisoned| {
        warn!("⚠️Mutex锁中毒，强制恢复访问");
        poisoned.into_inner() // 从中毒状态恢复数据访问
    });

    db_obj
        .conn
        .as_mut()
        .ok_or_else(|| {
            error!("取得rusqlite::Connection 可变访问出错");
            rusqlite::Error::InvalidQuery
        })?
        .execute(
            "INSERT INTO table_test (key, value) VALUES (?1, ?2)",
            [key, value],
        )?;

    info!("插入table_test 成功。key:{} value:{}", key, value);

    Ok(())
}

// 读取数据
fn query_data(key: &str) -> Result<String> {
    let db = global::get_global_db()?;
    let conn = db.lock().unwrap();
    let value: String = conn
        .conn
        .as_ref()
        .ok_or_else(|| {
            error!("取得rusqlite::Connection 出错");
            rusqlite::Error::InvalidQuery
        })?
        .query_row(
            "SELECT value FROM table_test WHERE key = ?1",
            [key],
            |row| row.get(0),
        )?;
    Ok(value)
}
