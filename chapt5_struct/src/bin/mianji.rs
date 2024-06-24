use std::fmt;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area_method(&self) -> u32 {
        self.height * self.width
    }

    // 可以选择将方法的名称与结构中的一个字段相同
    // 与字段同名的方法将被定义为只返回字段中的值，而不做其他事情
    // Getters 很有用，因为你可以把字段变成私有的，但方法是公共的，
    // 这样就可以把对字段的只读访问作为该类型公共 API 的一部分

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    // 带有更多参数的方法
    // 如果 self （第一个 Rectangle）能完全包含第二个长方形则返回 true；否则返回 false
    pub fn can_hold(&self, another: &Rectangle) -> bool {
        self.width > another.width && self.height > another.height
    }
}

// 多个impl快
impl Rectangle {
    // 所有在 impl 块中定义的函数被称为 关联函数（associated functions）
    // 可以理解为 不以self为第一个参数 就是关联函数  因为它们并不作用于一个结构体的实例

    // 关联函数经常被用作返回一个结构体新实例的构造函数
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "The rectangle width is {} height is {}",
            self.width, self.height
        )
    }
}

fn main() {
    let width = 30;
    let height = 50;
    println!(
        "The area of the rectangle is {} square pixels.",
        area(width, height)
    );

    let rectangle = Rectangle {
        width: 20,
        height: 40,
    };
    println!("{}", rectangle);
    println!("{:#?}", rectangle);

    println!(
        "The area of the rectangle is {} square pixels.",
        area_struct(&rectangle)
    );

    // 调用结构体的方法
    println!("调用方法 长方体的面积是{}", rectangle.area_method());

    // 调用 getter 方法
    println!(
        "调用 getter 方法 width={}, height={}",
        rectangle.width(),
        rectangle.height()
    );

    // Rust 有一个叫 自动引用和解引用（automatic referencing and dereferencing）的功能
    // 当使用 object.something() 调用方法时，Rust 会自动为 object 添加 &、&mut 或 * 以便使 object 与方法签名匹
    // 下面两行等价
    // rectangle.area_method();
    // (&rectangle).area_method();

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let square = Rectangle::square(10);
    println!("正方形 {:?}", square);

    dbg!(rectangle);
}

// 访问对结构体的引用的字段不会移动字段的所有权
fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
