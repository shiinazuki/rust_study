use std::slice;

// 使用 extern 函数调用外部代码
extern "C" {
    fn abs(input: i32) -> i32;
}

// 从其它语言调用 Rust 函数
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

fn main() {
    /*
        不安全 Rust：用于当需要舍弃 Rust 的某些保证并负责手动维持这些保证
        高级 trait：与 trait 相关的关联类型，默认类型参数，完全限定语法（fully qualified syntax），超（父）trait（supertraits）和 newtype 模式
        高级类型：关于 newtype 模式的更多内容，类型别名，never 类型和动态大小类型
        高级函数和闭包：函数指针和返回闭包
        宏：定义在编译时定义更多代码的方式
    */

    // 不安全 Rust

    // 不安全的超能力（unsafe superpowers）
    /*
        解引用裸指针
        调用不安全的函数或方法
        访问或修改可变静态变量
        实现不安全 trait
        访问 union 的字段
    */

    // unsafe 关键字只是提供了那五个不会被编译器检查内存安全的功能

    // 解引用裸指针
    // 不安全 Rust 有两个被称为 裸指针（raw pointers）的类似于引用的新类型
    // 和引用一样，裸指针是不可变或可变的，分别写作 *const T 和 *mut T
    // 在裸指针的上下文中，不可变 意味着指针解引用之后不能直接赋值

    /*
        裸指针与引用和智能指针的区别在于
            允许忽略借用规则，可以同时拥有不可变和可变的指针，或多个指向相同位置的可变指针
            不保证指向有效的内存
            允许为空
            不能实现任何自动清理功能
    */

    // 从引用同时创建不可变和可变裸指针
    let mut num = 5;

    // 可以在安全代码中 创建 裸指针
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    println!("r1={:?}, r2={:?}", r1, r2);

    // 只是不能在不安全块之外 解引用 裸指针
    // 对裸指针使用解引用运算符 *，这需要一个 unsafe 块
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    // 创建一个指向任意内存地址的裸指针
    // 编译器可能会优化掉这个内存访问，或者程序可能会出现段错误（segmentation fault）
    // 通常没有好的理由编写这样的代码，不过却是可行的
    let address = 0x012345usize;
    let _r = address as *const i32;

    // 创建一个指针不会造成任何危险；只有当访问其指向的值时才有可能遇到无效的值

    // 调用不安全函数或方法
    unsafe {
        dangerous();
    }

    // 创建不安全代码的安全抽象
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = split_at_mut(r, 3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    // 使用 extern 函数调用外部代码
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    // 访问或修改可变静态变量
    println!("name is: {}", HELLO_WORLD);

    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }

    // 实现不安全 trait

    // 访问联合体中的字段
}

unsafe trait Foo {}

unsafe impl Foo for i32 {}

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

// 访问和修改可变静态变量都是 不安全 的
static mut COUNTER: u32 = 0;

// 常量与不可变静态变量的一个微妙的区别是静态变量中的值有一个固定的内存地址
// 常量则允许在任何被用到的时候复制其数据。另一个区别在于静态变量可以是可变的
static HELLO_WORLD: &str = "Hello, world!";

// 何使用 unsafe 块，裸指针和一些不安全函数调用来实现 split_at_mut
fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    // Rust 的借用检查器不能理解我们要借用这个 slice 的两个不同部分：它只知道我们借用了同一个 slice 两次
    // (&mut values[..mid], &mut values[mid..])

    unsafe {
        (
            // slice::from_raw_parts_mut 函数获取一个裸指针和一个长度来创建一个 slice
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

// 不安全函数体也是有效的 unsafe 块，所以在不安全函数中进行另一个不安全操作时无需新增额外的 unsafe 块
unsafe fn dangerous() {
    println!("This is unsafe function");
}
