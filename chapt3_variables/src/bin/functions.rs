// 函数定义也是语句 语句不返回值
fn main() {
    println!("Hello, world!");

    another_function();

    another_param(13, 'h');

    // this is statements  这是语句
    let _y = 6;

    // let x = (let y  = 6);

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);

    let y = five();
    println!("The value of y is:{y}");

    let y = plus_one(5);
    println!("The value of y is:{y}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn five() -> i32 {
    5
}

fn another_param(x: i32, unit_label: char) {
    println!("The value of x is {}{}", x, unit_label);
}

fn another_function() {
    println!("Another function.");
}
