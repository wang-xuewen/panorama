use std::fmt::{Debug, Display};

use rusqlite::ToSql;

trait Greet {
    fn say_hello(&self);

    // 带有默认实现的方法
    fn say_goodbye(&self) {
        println!("Goodbye!");
    }
}

struct Person {
    name: String,
}
impl Greet for Person {
    fn say_hello(&self) {
        println!("Hello, my name is {}!", self.name);
    }
}

pub fn use_trait() {
    let person = Person {
        name: "Alice".to_string(),
    };
    person.say_hello();
    person.say_goodbye();
}
