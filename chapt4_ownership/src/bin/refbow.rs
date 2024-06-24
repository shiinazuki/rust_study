fn main() {
    // 引用

    // 引用的规则

    // 在任意给定时间，要么 只能有一个可变引用，要么 只能有多个不可变引用。
    // 引用必须总是有效的。

    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{s1}' is {len}.");

    let mut s = String::from("hello");
    change(&mut s);

    // 可变引用有一个很大的限制：如果有一个对该变量的可变引用，就不能再创建对该变量的引用
    // 这一限制以一种非常小心谨慎的方式允许可变性，防止同一时间对同一数据存在多个可变引用

    let r1 = &mut s;
    // 这里会编译错误  因为存在 对同一变量的多次借用
    // let r2 = &mut s;
    // println!("{}, {}", r1, r2);    let _r1 = &mut s;

    // println! 可以消耗掉引用 并且不会获得所有权
    println!("{}, ", r1);

    //  由于上一个引用已经被使用  所以可以再次借用
    let r2 = &mut s;
    println!("{}, ", r2);

    // 也可以创建一个新的作用域
    let mut s = String::from("hello");

    {
        let _r1 = &mut s;
    } // r1 在这里离开了作用域，所以我们完全可以创建一个新的引用

    let _r2 = &mut s;

    // Rust 在同时使用可变与不可变引用时也采用的类似的规则
    // 我们不能在拥有 不可变引用 的同时拥有 可变引用。
    let mut _s = String::from("hello");

    // let r1 = &s; // 没问题
    // let r2 = &s; // 没问题
    // let r3 = &mut s; // 大问题
    // println!("{}, {}, and {}", r1, r2, r3);

    // 一个引用的作用域从声明的地方开始一直持续到最后一次使用为止
}

// 可变借用
fn change(s: &mut String) {
    s.push_str(" world");
}

// 将创建一个引用的行为称为 借用（borrowing）

// 以一个对象的引用作为参数而不是获取值的所有权
fn calculate_length(s: &String) -> usize {
    s.len()
}
