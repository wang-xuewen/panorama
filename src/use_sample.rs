use crate::common::global;
use anyhow::Result;

pub fn use_sqlite() -> Result<()> {
    insert_data("", "")?;
    query_data("")?;
    Ok(())
}

// 写入数据
fn insert_data(key: &str, value: &str) -> Result<()> {
    let db = global::get_global_db()?;
    let mut conn = db.lock().unwrap();
    conn.conn.as_mut().unwrap().execute(
        "INSERT INTO table (key, value) VALUES (?1, ?2)",
        [key, value],
    )?;
    Ok(())
}

// 读取数据
fn query_data(key: &str) -> Result<String> {
    let db = global::get_global_db()?;
    let conn = db.lock().unwrap();
    let value: String = conn.conn.as_ref().unwrap().query_row(
        "SELECT value FROM table WHERE key = ?1",
        [key],
        |row| row.get(0),
    )?;
    Ok(value)
}
