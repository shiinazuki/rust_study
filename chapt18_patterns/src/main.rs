fn main() {
    // 模式与模式匹配

    // match 分支
    let x = Some(2);
    match x {
        Some(i) => Some(i + 1),
        None => None,
    };

    // match 表达式必须是 穷尽（exhaustive）的，意为 match 表达式所有可能的值都必须被考虑到

    // if let 条件表达式
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

    // while let 条件循环
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    // for 循环
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    // let 语句
    let (_x, _y, ..) = (1, 2, 3);

    // 函数参数

    // 模式有两种形式：refutable（可反驳的）和 irrefutable（不可反驳的）
    // 能匹配任何传递的可能值的模式被称为是 不可反驳的（irrefutable）
    // 对某些可能的值进行匹配会失败的模式被称为是 可反驳的（refutable）

    let p = Point { x: 0, y: 7 };
    match p {
        Point { x, y: 0 } => println!("on the x axis at {}", x),
        Point { x: 0, y } => println!("on the x axis at {}", y),
        Point { x, y } => println!("on the x axis:  ({}, {})", x, y),
    }

    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {setting_value:?}");


        let s = Some(String::from("Hello!"));

    if let Some(_) = s {
        println!("found a string");
    }

    println!("{s:?}");


    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {n}"),
        _ => println!("Default case, x = {x:?}"),
    }

    println!("at the end: x = {x:?}, y = {y}");

}

struct Point {
    x: i32,
    y: i32,
}
