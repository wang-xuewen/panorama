use crate::sqlite_sample::sqlite_c::SqliteCrud;
use anyhow::{bail, Result};
use rusqlite::params;

pub struct User {
    pub id: i32,
    pub name: String,
    pub age: i32,
}

impl User {
    pub fn new(id: i32, name: String, age: i32) -> Result<Self> {
        Ok(Self { id, name, age })
    }
    /// 初始化表结构
    pub fn init_table(&self, db: &SqliteCrud) -> Result<()> {
        if let Some(conn) = &db.conn {
            conn.execute(
                "CREATE TABLE IF NOT EXISTS users (
                        id INTEGER PRIMARY KEY AUTOINCREMENT,
                        name TEXT NOT NULL,
                        age INTEGER NOT NULL
                    )",
                [],
            )?;
            Ok(())
        } else {
            bail!("Connection is None")
        }
    }

    /// 插入一个用户
    pub fn insert_user(&self, db: &SqliteCrud, name: &str, age: i32) -> Result<()> {
        if let Some(conn) = &db.conn {
            conn.execute(
                "INSERT INTO users (name, age) VALUES (?1, ?2)",
                params![name, age],
            )?;
            Ok(())
        } else {
            bail!("Connection is None")
        }
    }

    /// 查询所有用户
    pub fn query_users(&self, db: &SqliteCrud) -> Result<Vec<User>> {
        if let Some(conn) = &db.conn {
            let mut stmt = conn.prepare("SELECT id, name, age FROM users")?;
            let rows = stmt.query_map([], |row| {
                Ok(User {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    age: row.get(2)?,
                })
            })?;

            let mut users = Vec::new();
            for user in rows {
                users.push(user?);
            }
            Ok(users)
        } else {
            bail!("Connection is None")
        }
    }

    /// 更新用户信息
    pub fn update_user(&self, db: &SqliteCrud, id: i32, name: &str, age: i32) -> Result<()> {
        if let Some(conn) = &db.conn {
            conn.execute(
                "UPDATE users SET name = ?1, age = ?2 WHERE id = ?3",
                params![name, age, id],
            )?;
            Ok(())
        } else {
            bail!("Connection is None")
        }
    }

    /// 删除用户
    pub fn delete_user(&self, db: &SqliteCrud, id: i32) -> Result<()> {
        if let Some(conn) = &db.conn {
            conn.execute("DELETE FROM users WHERE id = ?1", params![id])?;
            Ok(())
        } else {
            bail!("Connection is None")
        }
    }

    // 使用事务
    // pub fn transfer_money(
    //     &self,
    //     from: i32,
    //     to: i32,
    //     amount: i32,
    // ) -> Result<(), rusqlite::Error> {
    //     let conn = self.get_conn();
    //     let tx = conn.transaction()?;

    //     // 执行转账操作
    //     tx.execute(
    //         "UPDATE accounts SET balance = balance - ?1 WHERE id = ?2",
    //         [amount, from],
    //     )?;
    //     tx.execute(
    //         "UPDATE accounts SET balance = balance + ?1 WHERE id = ?2",
    //         [amount, to],
    //     )?;

    //     tx.commit()?;
    //     Ok(())
    // }
}
