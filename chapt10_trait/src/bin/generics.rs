fn main() {
    // 使用泛型定义自己的类型、函数和方法

    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q', 'z'];
    let result = char_max(&char_list);
    println!("The largest char is {}", result);

    // 使用泛型函数
    let number_list = vec![34, 50, 25, 100, 65];
    let result = max_one(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = max_one(&char_list);
    println!("The largest char is {result}");

    // 泛型结构体
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    println!("integer={:?}, float={:?}", integer, float);
    println!("x={}, y={}", float.x, float.y);

    // let wont_work = Point {x : 5, y: 4.0}; // 错误的
    let wont_work = De { x: 5, y: 4.0 };
    println!("wont_work={:?}", wont_work);

    let point = Point::new(10, 20);
    println!("point的new={:?}", point);

    let _option = Option::Some(10);
    let _option = Option::None("err");

    let _res: Result<i32, &str> = Result::Ok(10);
    let _res: Result<i64, &str> = Result::Err("err");

    let p1 = Po { x: 5, y: 10.4 };
    let p2 = Po { x: "hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3={:?}", p3);
    
}

#[derive(Debug)]
struct Po<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Po<X1, Y1> {
    // 用 self 的 Point 类型的 x 值（类型 X1）和参数的 Point 类型的 y 值（类型 Y2）来创建一个新 Point 类型的实例
    fn mixup<X2, Y2>(self, other: Po<X2, Y2>) -> Po<X1, Y2> {
        Po {
            x: self.x,
            y: other.y,
        }
    }
}

// 枚举定义中的泛型
enum Option<T> {
    Some(T),
    None(T),
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

#[derive(Debug)]
struct De<T, U> {
    x: T,
    y: U,
}

// 结构体定义中的泛型
//  定义一个可以存放任何类型的 x 和 y 坐标值的结构体 Pointh
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    // 方法定义中的泛型
    fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
    fn y(&self) -> &T {
        &self.y
    }
}

impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// 在函数定义中使用泛型 来消除重复代码
fn max_one<T>(list: &[T]) -> &T
where
    T: std::cmp::PartialOrd,
{
    let mut max = &list[0];
    for item in list {
        if item > max {
            max = item
        }
    }
    max
}

// 提取函数来减少重复
fn largest(list: &[i32]) -> &i32 {
    let mut max = &list[0];
    for number in list {
        if number > max {
            max = number;
        }
    }
    max
}

fn char_max(chars: &[char]) -> &char {
    let mut max = &chars[0];
    for ch in chars {
        if ch > max {
            max = ch;
        }
    }
    max
}
