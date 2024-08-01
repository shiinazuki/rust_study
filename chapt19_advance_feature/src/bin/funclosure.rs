fn main() {
    // 高级函数与闭包

    // 函数指针  fn 被称为 函数指针（function pointer）
    // 通过函数指针允许我们使用函数作为另一个函数的参数
    let answer = do_twice(add_one, 5);
    println!("The answer is: {}", answer);

    // 函数指针实现了所有三个闭包 trait（Fn、FnMut 和 FnOnce），所以总是可以在调用期望闭包的函数时传递函数指针作为参数

    // 使用 map 函数将一个数字 vector 转换为一个字符串 vector
    // 使用闭包
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings = list_of_numbers
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    println!("{:?}", list_of_strings);

    // 使用函数
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings = list_of_numbers
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<String>>();
    println!("{:?}", list_of_strings);

    // 可以指定构造函数作为接受闭包的方法的参数
    let list_of_statuses = (0_u32..20).map(Status::Value).collect::<Vec<Status>>();
    println!("{:?}", list_of_statuses);

    // 返回闭包

    // 闭包表现为 trait，这意味着不能直接返回闭包
    

}


// 错误指向了 Sized trait！Rust 并不知道需要多少空间来储存闭包
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}


#[derive(Debug)]
enum Status {
    Value(u32),
    Stop,
}

// 指定参数为函数指针的语法类似于闭包
// do_twice 函数获取两个参数：一个指向任何获取一个 i32 参数并返回一个 i32 的函数指针，和一个 i32 值
// do_twice 函数传递 arg 参数调用 f 函数两次，接着将两次函数调用的结果相加
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn add_one(x: i32) -> i32 {
    x + 1
}