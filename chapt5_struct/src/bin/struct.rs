// 存储账号信息的结构体

#[derive(Default, Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

impl User {
    fn new() -> User {
        User {
            ..Default::default()
        }
    }
}

// 使用没有命名字段的 元组结构体 来创建不同的类型

#[derive(Debug)]
struct Color(i32, i32, i32);

#[derive(Debug)]
struct Point(i32, i32, i32);

// 没有任何字段的类单元结构体
// 类单元结构体常常在你想要在某个类型上实现 trait 但不需要在类型中存储数据的时候发挥作用
struct AlwaysEqual;

fn main() {
    // 结构体的定义和实例化
    let mut user1 = User {
        active: true,
        username: String::from("shiina"),
        email: String::from("xx@xxx.com"),
        sign_in_count: 49,
    };
    println!("user1={:#?}", user1);

    // 如果结构体的实例是可变的，我们可以使用点号并为对应的字段赋值
    user1.sign_in_count = 99;

    // 从结构体中获取某个特定的值，可以使用点号
    println!(
        "active={}, username={}, email={}, sign_in_count={}",
        user1.active, user1.username, user1.email, user1.sign_in_count
    );

    let user1 = User::new();
    println!("user1={:#?}", user1);

    // 结构体更新语法
    let user2 = User {
        email: String::from("xxxx@xxxx.com"),
        // username: String::from("shiina"),
        // 结构更新语法就像带有 = 的赋值，因为它移动了数据
        // 无法被 Copy 的数据被移动走了 所以user1在之后就不能在使用了
        // 如果只移动了可以被Copy的数据 user1还可以继续使用
        ..user1
    };
    println!("user2={:#?}", user2);
    // 无法被Copy的类型 move 了  编译错误
    // println!("user1={:#?}", user1);

    // 元组结构体
    // 元组结构体实例类似于元组，你可以将它们解构为单独的部分，也可以使用 . 后跟索引来访问单独的值
    let black = Color(0, 0, 0);
    println!("{:#?}", black);
    println!("0={}, 1={}, 2={}", black.0, black.1, black.2);
    let Color(a, b, c) = black;
    println!("a={}, b={}, c={}", a, b, c);

    println!("========================================================");

    let origin = Point(0, 0, 0);
    println!("{:#?}", origin);
    println!("0={}, 1={}, 2={}", origin.0, origin.1, origin.2);
    let Point(a, b, c) = origin;
    println!("a={}, b={}, c={}", a, b, c);

    // 类单元结构体
    let _subject = AlwaysEqual;

    // 结构体存储数据的引用 需要加上生命周期
    let person = Person {
        active: true,
        username: "shiina",
        email: "xxxx@xxxxx.com",
    };
    println!("people={:#?}", person);

    let person = Person::new("shiina", "xx@xx.com");
    println!("people={:#?}", person);
}

// 可以使结构体存储被其他对象拥有的数据的引用，不过这么做的话需要用上 生命周期

#[derive(Debug)]
struct Person<'a> {
    active: bool,
    username: &'a str,
    email: &'a str,
}

impl<'a> Person<'a> {
    fn new(username: &'a str, email: &'a str) -> Person<'a> {
        Person {
            active: true,
            username,
            email,
        }
    }
}
