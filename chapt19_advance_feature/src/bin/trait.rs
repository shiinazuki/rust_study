use std::{fmt, ops::Add};

fn main() {
    // 高级 trait

    // 关联类型在 trait 定义中指定占位符类型

    // 默认泛型类型参数和运算符重载
    // 当使用泛型类型参数时，可以为泛型指定一个默认的具体类型

    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    assert_eq!(Millimeters(5000) + Meters(3), Millimeters(8000));

    // 完全限定语法与消歧义：调用相同名称的方法
    let person = Human;
    // 默认调用直接实现在类型上的方法
    person.fly();

    Pilot::fly(&person);
    Wizard::fly(&person);

    // 因为 fly 方法获取一个 self 参数，如果有两个 类型 都实现了同一 trait，Rust 可以根据 self 的类型计算出应该使用哪一个 trait 实现

    // 不是方法的关联函数没有 self 参数
    // 当存在多个类型或者 trait 定义了相同函数名的非方法函数时  需要使用 完全限定语法（fully qualified syntax）
    println!("A baby dog is called a {}", Dog::baby_name());
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());

    // 父 trait 用于在另一个 trait 中使用某 trait 的功能
    // 编写一个依赖另一个 trait 的 trait 定义：对于一个实现了第一个 trait 的类型，要求这个类型也实现了第二个 trait

    let way = Way { x: 1, y: 3 };
    way.outline_print();


    // newtype 模式用以在外部类型上实现外部 trait
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);

}


// 孤儿规则（orphan rule） 只要 trait 或类型对于当前 crate 是本地的话就可以在此类型上实现该 trait
// 一个绕开这个限制的方法是使用 newtype 模式（newtype pattern），它涉及到在一个元组结构体中创建一个新类型
// 这个元组结构体带有一个字段作为希望实现 trait 的类型的简单封装。接着这个封装类型对于 crate 是本地的，这样就可以在这个封装上实现 trait

// 简单理解就是套一层

// 在 Vec<T> 上实现 Display

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}



#[derive(Debug)]
struct Way {
    x: i32,
    y: i32,
}

impl fmt::Display for Way {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl OutLinePrint for Way {}

// 通过在 trait 定义中指定 OutlinePrint: Display 来做到这一点。这类似于为 trait 增加 trait bound
trait OutLinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {output} *");
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

// 默认参数类型主要用于如下两个方面：
// 扩展类型而不破坏现有代码
// 在大部分用户都不需要的特定情况进行自定义

// 将毫米与米相加  并让 Add 的实现正确处理转换
#[derive(Debug, PartialEq)]
struct Millimeters(u32);

#[derive(Debug, PartialEq)]
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;
    fn add(self, other: Meters) -> Self::Output {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

// 尖括号中的 Rhs=Self：这个语法叫做 默认类型参数（default type parameters）
// 如果实现 Add trait 时不指定 Rhs 的具体类型，Rhs 的类型将是默认的 Self 类型，也就是在其上实现 Add 的类型
// trait Add<Rhs=Self> {
//     type Output;

//     fn add(self, rhs: Rhs) -> Self::Output;
// }

// 在 Point 结构体上实现 Add trait 来重载 + 运算符，就可以将两个 Point 实例相加了
#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Self) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

struct Counter {}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        Some(2)
    }
}

// 当 trait 有泛型参数时，可以多次实现这个 trait，每次需改变泛型参数的具体类型
// 通过关联类型，则无需标注类型，因为不能多次实现这个 trait

// 区别在于 像下面那样使用泛型时，则不得不在每一个实现中标注类型

// 这个语法类似于泛型。为什么 Iterator trait 不像下面这样定义
pub trait IteratorE<T> {
    fn next(&mut self) -> Option<T>;
}
