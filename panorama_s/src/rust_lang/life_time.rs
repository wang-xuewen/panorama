use log::info;


// 生命周期 'a 的含义：返回的引用将至少与参数 x 和 y 中生命周期较短的那个一样长。
fn longest<'a>(x:&'a str,y:&'a str) -> &'a str{
    if x.len() > y.len() {
        x
    }else {
        y
    }
}


// 结构体中的生命周期
// 结构体包含引用时，必须声明生命周期
struct ImportantExcept<'a> {
    part: &'a str,
}
impl <'a> ImportantExcept<'a> {
    fn level(&self)->i32 {
        3
    }
    fn announce_and_return_part(&self,announcement:&str)->&str{
        info!("Attention please: {}",announcement);
        self.part
    }
}
// 生命周期省略规则
// 生命周期省略规则就是关于如何从输入生命周期推断出输出生命周期的。
// 目的是为了简洁：这些规则覆盖了函数签名的绝大多数常见模式，让你在写安全代码的同时，保持代码的清爽。
// 规则 1：每个是引用的参数都有自己独立的生命周期
// 规则 2：如果只有一个输入生命周期参数，那么这个生命周期会被赋给所有输出生命周期参数
// 规则 3：如果方法有多个输入生命周期参数，但其中一个是 &self 或 &mut self，那么 self 的生命周期会被赋给所有输出生命周期参数

// 此处应用了规则1 和规则2，所以不需要再手动声明生命周期，编译器会根据规则自动识别生命周期
fn first_word(s:&str) -> &str{
    let bytes = s.bytes();
    for (i,item) in bytes.into_iter().enumerate() {
        if item == b' '{
            return &s[0..i];
        }
    }
    &s[..]
}
// 此处应用了规则3
impl <'a> ImportantExcept<'a> {
    fn get_part(&self,delimiter:&str) ->&str {
        self.part
    }
}

// 静态生命周期
// 'static 生命周期表示引用在整个程序运行期间都有效
fn static_lift_time() -> &'static str {
    // 字符串字面值有 'static 生命周期
    let s:&'static str = "I have a static lifetime.";

    // 也可以创建静态变量
    static HELLO_WORLD: &str = "Hello world!";
    info!("{}",HELLO_WORLD);

    "This string lives for the entire programme"
}

// 多个生命周期参数
fn longest_with_an_announcement<'a,'b> (x: &'a str,y: &'a str, ann:&'b str) -> &'a  str 
where  'b:'a {
    info!("Announcement:{}",ann);
    if x.len()> y.len() {
        ann
    }else {
        y
    }
}

fn err_sample() {
    // 示例1：悬垂引用
    // let r;
    // {
    //     let x = 5;
    //     r = &x; // ❌ 错误：`x` 的生命周期不够长
    // }
    // println!("r: {}", r);

    // 示例2：函数返回局部变量的引用
    // fn dangle() -> &String {
    //     let s = String::from("hello");
    //     &s // ❌ 错误：返回局部变量的引用
    // }

    // 正确的做法：返回所有权而不是引用
    fn no_dangle() -> String {
        let s = String::from("hello");
        s // ✅ 正确：返回所有权
    }
}

pub fn use_life_time() {
    // 生命周期简单例子
    let string1 = String::from("long string is long");
    let string2 = "xyz";
    let ret = longest(string1.as_str(), string2);
    info!("The longest string is {}",ret);

    // 结构体生命周期
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcept {
        part:first_sentence,
    };
    info!("First sentence： {}",i.announce_and_return_part(novel.as_str()));
    info!("Level: {}",i.level());
    
    // static_ref 可以在程序的任何地方使用(还需要注意可见性，那是另外的空间概念)
    // 永远不会悬垂（dangling）
    // 不需要担心它的生命周期问题
    let static_ref = static_lift_time();
    info!("{}",static_ref);

    // 多生命周期
    let announcement = "Finding the longest string!";
    let ret2 = longest_with_an_announcement(string1.as_str(), string2, announcement);
    info!("The longest string is {}",ret2)

}