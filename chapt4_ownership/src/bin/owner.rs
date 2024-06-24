// 所有权规则
// Rust 中的每一个值都有一个 所有者（owner）。
// 值在任一时刻有且只有一个所有者。
// 当所有者（变量）离开作用域，这个值将被丢弃。
fn main() {
    let _s = String::from("hello");

    let mut s = String::from("hello");

    s.push_str(", world");

    println!("{}", s);

    let x = 5;
    let _y = x;

    // 将 s1 赋值给 s2，String 的数据被复制了，只是从栈上拷贝了它的指针、长度和容量。并没有复制指针指向的堆上数据
    let s1 = String::from("hello");
    let _s2 = s1;

    // s1的所有权移动(move)给s2了 所以不可使用
    // println!("{}", s1);
    println!("{}", _s2);

    // println 只借用传递的值 不会移动(move)变量的所有权
    let s3 = String::from("hello");
    println!("{}", s3);
    let s4 = s3;
    println!("{}", s4);

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");

    let s1 = gives_ownership();
    println!("{}", s1);

    let s2 = String::from("hello");

    let s3 = takes_and_gives_back(s2);

    println!("{}", s3);

    // 计算字符串的长度
    let s1 = String::from("hello, world");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

// 使用元组返回多个值
// 一般不这样用
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn gives_ownership() -> String {
    // gives_ownership 会将
    // 返回值移动给
    // 调用它的函数

    let some_string = String::from("yours"); // some_string 进入作用域。

    some_string // 返回 some_string
                // 并移出给调用的函数
                //
}

// takes_and_gives_back 将传入字符串并返回该值
fn takes_and_gives_back(a_string: String) -> String {
    // a_string 进入作用域
    //

    a_string // 返回 a_string 并移出给调用的函数
}
