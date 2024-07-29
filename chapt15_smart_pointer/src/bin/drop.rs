fn main() {
    // 使用 Drop Trait 运行清理代码

    // 指定在值离开作用域时应该执行的代码的方式是实现 Drop trait
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };

    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");

    // Rust 并不允许我们主动调用 Drop trait 的 drop 方法
    // 因为 Rust 仍然会在 main 的结尾对值自动调用 drop，这会导致一个 double free 错误
    // c.drop();

    // 通过 std::mem::drop 提早丢弃值
    // 使用 std::mem::drop 函数   std::mem::drop 位于 prelude
    drop(c);

    println!("CustomSmartPointer dropped before the end of main.");
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data '{}'!", self.data);
    }
}
