use std::fmt::{Debug, Display};

// ========== 基本 Trait 定义 ==========
// 定义一个基本 trait
trait Greet {
    // 必须实现的方法
    fn say_hello(&self);

    // 带有默认实现的方法
    fn say_goodbye(&self) {
        println!("Goodbye!");
    }
}

// ========== 实现 Trait ==========
struct Person {
    name: String,
}

impl Greet for Person {
    fn say_hello(&self) {
        println!("Hello, my name is {}!", self.name);
    }

    // 使用默认的 say_goodbye 实现
}

// ========== Trait 对象 ==========
fn greet_someone(g: &dyn Greet) {
    g.say_hello();
    g.say_goodbye();
}

// ========== 关联类型 ==========
trait Container {
    type Item; // 关联类型

    fn get(&self, index: usize) -> Option<&Self::Item>;
    fn add(&mut self, item: Self::Item);
}

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

// ========== 泛型 Trait ==========
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

// ========== Trait 约束 ==========
// 使用 trait 作为泛型约束
fn print_greeting<T: Greet>(item: T) {
    item.say_hello();
}

// 多重约束
fn clone_and_print<T: Greet + Clone>(item: T) {
    let cloned = item.clone();
    cloned.say_hello();
}

// where 子句
fn complex_function<T, U>(t: T, u: U)
where
    T: Display + Clone,
    U: Clone + Debug,
{
    println!("T: {}, U: {:?}", t, u.clone());
}

// ========== 自动 Trait ==========
// Send 和 Sync 是自动 trait
fn is_send<T: Send>(t: T) {}
fn is_sync<T: Sync>(t: T) {}

// ========== 条件实现 ==========
// 为实现了特定 trait 的类型实现 trait
trait Printable {
    fn print(&self);
}

impl<T: Display> Printable for T {
    fn print(&self) {
        println!("Printable: {}", self);
    }
}

// ========== Supertrait ==========
// 一个 trait 依赖于另一个 trait
trait Loggable: Display {
    fn log(&self) {
        println!("Log: {}", self);
    }
}

impl Loggable for Person {}

// 为 Person 实现 Display 以便使用 Loggable
impl Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Person({})", self.name)
    }
}

// ========== 标记 Trait ==========
trait MarkerTrait {}

impl MarkerTrait for Person {}

// ========== 示例使用 ==========
fn main() {
    // 基本 trait 使用
    let person = Person {
        name: "Alice".to_string(),
    };
    person.say_hello();
    person.say_goodbye();

    // trait 对象
    greet_someone(&person);

    // 关联类型
    let mut container = MyVec {
        items: vec![1, 2, 3],
    };
    container.add(4);
    println!("Item at index 2: {:?}", container.get(2));

    // 泛型 trait
    println!("As String: {}", person.convert::<String>());
    println!("As u32: {}", person.convert::<u32>());

    // trait 约束
    print_greeting(person);

    // 条件实现
    "Hello".print();
    42.print();

    // supertrait
    let person2 = Person {
        name: "Bob".to_string(),
    };
    person2.log();

    // 自动 trait
    is_send(person2);
    // is_send(&person); // 这会编译错误，因为 &Person 不是 Send

    // 标记 trait
    fn accept_marker<T: MarkerTrait>(_t: T) {}
    accept_marker(Person {
        name: "Charlie".to_string(),
    });
}
