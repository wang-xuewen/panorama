// let db = SqliteCrud::new("test.db")?;
//     db.init_table()?;

//     db.insert_user("Alice", 28)?;
//     db.insert_user("Bob", 32)?;

//     println!("All users:");
//     for user in db.query_users()? {
//         println!("{:?}", user);
//     }

//     db.update_user(1, "Alice Updated", 29)?;
//     db.delete_user(2)?;

//     println!("After update and delete:");
//     for user in db.query_users()? {
//         println!("{:?}", user);
//     }

//     Ok(())
