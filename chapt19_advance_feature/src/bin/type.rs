fn main() {
    // 高级类型

    // 为了类型安全和抽象而使用 newtype 模式

    // 类型别名用来创建类型同义词   使用 type 关键字来给予现有类型另一个名字
    type Kiolmeters = i32;

    // Kilometers 类型的值将被完全当作 i32 类型值来对待
    let x = 5;
    let y: Kiolmeters = 5;
    println!("x + y = {}", x + y);

    // 类型别名的主要用途是减少重复。例如，可能会有很长的类型
    // Box<dyn Fn() + Send + 'static>

    let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));

    fn takes_long_type(f: Box<dyn Fn() + Send + 'static>) {}

    fn returns_long_type() -> Box<dyn Fn() + Send + 'static> {
        todo!()
    }

    // 类型别名通过减少项目中重复代码的数量来使其更加易于控制
    type Thunk = Box<dyn Fn() + Send + 'static>;

    let f: Thunk = Box::new(|| println!("hi"));

    fn takes_short_type(f: Thunk) {}

    fn returns_short_type() -> Thunk {
        todo!()
    }

    // 类型别名也经常与 Result<T, E> 结合使用来减少重复

    // 从不返回的 never type  !

    // 动态大小类型和 Sized trait

    // let s1: str = "Hello there!";
    // let s2: str = "How's it going?";

    // 必须将动态大小类型的值置于某种指针之后
}

// Rust 隐式的为每一个泛型函数增加了 Sized bound
// fn generic<T>(t: T) {}

// 实际上被当作如下处理：
// fn generic<T: Sized>(t: T) {}

// 泛型函数默认只能用于在编译时已知大小的类型。然而可以使用如下特殊语法来放宽这个限制
fn generic<T: ?Sized>(t: &T) {}

// 这种意义的 ?Trait 语法只能用于 Sized ，而不能用于任何其他 trait
// 另外注意我们将 t 参数的类型从 T 变为了 &T：因为其类型可能不是 Sized 的，所以需要将其置于某种指针之后。在这个例子中选择了引用

// 从不返回的函数被称为 发散函数（diverging functions）
fn bar() -> ! {
    loop {
        println!("and ever ");
    }
}
