use log::{error, info, warn};
use std::fmt::{Debug, Display};

use rusqlite::ToSql;

// 基本trait
trait Greet {
    fn say_hello(&self);

    // 带有默认实现的方法
    fn say_goodbye(&self) {
        info!("Goodbye!");
    }
}

struct Person {
    name: String,
}
impl Greet for Person {
    fn say_hello(&self) {
        info!("Hello, my name is {}!", self.name);
    }
}

// trait 对象
fn greet_someone(g: &dyn Greet) {
    g.say_hello();
    g.say_goodbye();
}

trait Container {
    type Item;

    fn get(&self, index: usize) -> Option<&Self::Item>;
    fn add(&mut self, item: Self::Item);
}

// 关联类型
struct MyVec<T> {
    items: Vec<T>,
}

impl<T> Container for MyVec<T> {
    type Item = T;
    fn get(&self, index: usize) -> Option<&T> {
        self.items.get(index)
    }
    fn add(&mut self, item: T) {
        self.items.push(item);
    }
}

// 泛型trait
trait Converter<T> {
    fn convert(&self) -> T;
}

impl Converter<String> for Person {
    fn convert(&self) -> String {
        format!("Person[name={}]", self.name)
    }
}

impl Converter<u32> for Person {
    fn convert(&self) -> u32 {
        self.name.len() as u32
    }
}

// trait 约束（使用泛型作为trait约束）
fn print_greeting<T: Greet>(item: T) {
    item.say_hello();
}

// 自动trait
struct MyData(u32); // 自动实现 Send（因为 u32 是 Send）
fn spawn_thread() {
    let data = MyData(42);
    std::thread::spawn(move || info!("Data :{}", data.0)); // 合法，因为 MyData 是 Send
}
fn is_sync<T: Sync>(t: T) {}

// super trait (一个 trait 依赖于另一个 trait)
trait Loggable: Display {
    fn log(&self) {
        info!("Log: {}", self)
    }
}
impl Loggable for Person {}
impl Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Person({})", self.name)
    }
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

    // 关联类型
    let mut container = MyVec {
        items: vec![1, 2, 3],
    };
    container.add(4);
    info!("Item at index 2: {:?}", container.get(2));

    // 泛型trait
    let result1: String = person.convert(); // 推断为 String
    let result2: u32 = person.convert(); // 推断为 u32
    info!("String: {}", result1); // Person[name=Charlie]
    info!("U32: {}", result2); // 7

    // trait 约束（使用泛型作为trait约束）
    print_greeting(person);

    // supertrait
    let person2 = Person {
        name: "Bob".to_string(),
    };
    person2.log();

    // 自动 trait
    is_sync(person2);
}
