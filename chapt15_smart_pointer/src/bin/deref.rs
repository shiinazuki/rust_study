use std::ops::Deref;

fn main() {
    // 通过 Deref trait 将智能指针当作常规引用处理

    let x = 4;
    let y = &x;

    assert_eq!(4, x);
    // 解引用 一旦解引用了 y，就可以访问 y 所指向的整型值 并可以比较
    assert_eq!(4, *y);

    // 像引用一样使用 Box<T>
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    // 等于上边
    assert_eq!(5, *(y.deref()));

    // 因为 Deref 强制转换，使用 MyBox<String> 的引用调用 hello 是可行的
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    // Rust 没有 Deref 强制转换则必须编写的代码
    hello(&(*m)[..]);

    // Deref 强制转换如何与可变性交互

    // Rust 在发现类型和 trait 实现满足三种情况时会进行 Deref 强制转换
    /*
        当 T: Deref<Target=U> 时从 &T 到 &U。
        当 T: DerefMut<Target=U> 时从 &mut T 到 &mut U。
        当 T: Deref<Target=U> 时从 &mut T 到 &U。
    */


}

// 函数和方法的隐式 Deref 强制转换
fn hello(name: &str) {
    println!("Hello, {}", name);
}

// 自定义智能指针

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// 通过实现 Deref trait 将某类型像引用一样处理
impl<T> Deref for MyBox<T> {
    // type Target = T; 语法定义了用于此 trait 的关联类型
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
