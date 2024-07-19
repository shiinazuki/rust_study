use std::fmt::Display;

fn main() {
    // 生命周期确保引用有效

    // 生命周期的主要目标是避免悬垂引用（dangling references）
    let _r;
    {
        let x = 5;
        _r = &x;
    } // x 在这里离开了作用域
      // 但是 r 还想引用 x 这是不允许的
      // println!("r: {}", r);
    {
        let x = 5;
        let r = &x;
        println!("r: {}", r);
    }

    // 函数中的泛型生命周期
    let str1 = String::from("abcd");
    let str2 = "xyz";

    let res = longest(&str1.as_str(), &str2);
    println!("The longest string is {}", res);

    // &i32        // 引用
    // &'a i32     // 带有显式生命周期的引用
    // &'a mut i32 // 带有显式生命周期的可变引用

    // 通过传递拥有不同具体生命周期的引用来限制 longest 函数的使用
    let str1 = String::from("long string is long");
    {
        let str2 = String::from("xyz");
        let res = longest(str1.as_str(), str2.as_str());
        println!("The longest string is {}", res);
    }

    // 不能编译
    // 为了保证 println! 中的 result 是有效的，str2 需要直到外部作用域结束都是有效的
    // Rust 知道这些是因为（longest）函数的参数和返回值都使用了相同的生命周期参数 'a。
    // let str1 = String::from("long str is long");
    // let res;
    // {
    //     let str2 = String::from("xyz");
    //     res = longest(str1.as_str(), str2.as_str());
    // }
    // println!("The longest string is {}", res);

    // 深入理解生命周期
    // 指定生命周期参数的正确方式依赖函数实现的具体功能

    // 生命周期语法是用于将函数的多个参数与其返回值的生命周期进行关联的

    // 存放引用的结构体
    let novel = String::from("Call me Ishmael. Some years age...");
    let first_sentence = novel.split('.').next().unwrap();
    // ImportantExcerpt 的实例不能比其 part 字段中的引用存在的更久
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("first={:?} i={:?}", first_sentence, i);

    // 生命周期省略（Lifetime Elision）

    // 每一个引用都有一个生命周期，而且我们需要为那些使用了引用的函数或结构体指定生命周期

    // !!!!!
    // 函数或方法的参数的生命周期被称为 输入生命周期（input lifetimes）
    // 返回值的生命周期被称为 输出生命周期（output lifetimes）
    // 编译器采用三条规则来判断引用何时不需要明确的注解
    /*
        1. 编译器为每一个引用参数都分配一个生命周期参数
            换句话说就是，函数有一个引用参数的就有一个生命周期参数：fn foo<'a>(x: &'a i32)，
            有两个引用参数的函数就有两个不同的生命周期参数，依此类推

        2. 如果只有一个输入生命周期参数，那么它被赋予所有输出生命周期参数：fn foo<'a>(x: &'a i32) -> &'a i32

        3. 如果方法有多个输入生命周期参数并且其中一个参数是 &self 或 &mut self，
           说明是个对象的方法 (method) 那么所有输出生命周期参数被赋予 self 的生命周期
    */

    // 所有的字符串字面值都拥有 'static 生命周期
    let _s: &'static str = "I have a static lifetime";
}

// 结合泛型类型参数、trait bounds 和生命周期
fn _longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 静态生命周期

// 方法定义中的生命周期注解
struct Import<'a> {
    str: &'a str,
}

impl<'a, 'b> Import<'a> {
    fn level(&self) -> i32 {
        3
    }
    // rust会给两个参数分别赋各自的生命周期
    // 由于有一个参数是&self  返回值被赋予 &self的生命周期  这样推断出了所有声明周期了
    fn announce_and_return_part(&'a self, announcement: &'b str) -> &'a str {
        println!("Attention please: {announcement}");
        self.str
    }
}

fn _first_word<'a>(s: &'a str) -> &'a str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

// 结构体定义中的生命周期注解

// 一个存放引用的结构体 所以其定义需要生命周期注解
#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// 如果返回的引用 没有 指向任何一个参数，那么唯一的可能就是它指向一个函数内部创建的值
// 然而它将会是一个悬垂引用，因为它将会在函数结束时离开作用域
// 这种情况，最好的解决方案是返回一个有所有权的数据类型 而不是一个引用，这样函数调用者就需要负责清理这个值了。
// fn longest_in<'a>(x: &str, y: &str) -> &'a str {
//     let result = String::from("really long string");
//     &result.as_str()
// }

// 总是返回第一个参数 就不需要为第二个参数指定一个声明周期
fn longest_one<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

// 返回两个字符串 slice 中较长者的函数

// 这两个参数和返回的引用存活的一样久。（两个）参数和返回的引用的生命周期是相关的
// 函数返回的 引用的 生命周期 与 函数参数所引用的 值的生命周期的较小者一致 !!!!!
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 当具体的引用被传递给 longest 时，被 'a 所替代的具体生命周期是 x 的作用域与 y 的作用域相重叠的那一部分
// 换一种说法就是泛型生命周期 'a 的具体生命周期等同于 x 和 y 的生命周期中较小的那一个
// 因为 用相同的生命周期参数 'a 标注了返回的引用值，所以返回的引用值就能保证在 x 和 y 中较短的那个生命周期结束之前保持有效

// 生命周期注解告诉 Rust 多个引用的泛型生命周期参数如何相互联系的
// 如果函数有一个生命周期 'a 的 i32 的引用的参数 first。还有另一个同样是生命周期 'a 的 i32 的引用的参数 second。
// 这两个生命周期注解意味着引用 first 和 second 必须与这泛型生命周期存在得一样久。
