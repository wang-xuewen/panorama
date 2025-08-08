use log::{error, info, warn};
use std::fmt::{Debug, Display};

use rusqlite::ToSql;

// 基本trait
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

// trait 对象
fn greet_someone(g: &dyn Greet) {
    g.say_hello();
    g.say_goodbye();
}

pub fn use_trait() {
    // 基本trait
    info!("基本 trait");
    let person = Person {
        name: "Alice".to_string(),
    };
    person.say_hello();
    person.say_goodbye();

    // trait 对象
    info!("trait 对象");
    greet_someone(&person);
}
