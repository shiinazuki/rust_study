use std::usize;

fn main() {
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);

    let _f = 43.2;

    let _c = 'H';

    let _b = true;

    let tup: (i32, f64, u8) = (23_i32, 11.9, 22_u8);

    let (x, y, z) = tup;
    println!("解构的元组是 x ={} y ={} z ={}", x, y, z);

    let tup: (i32, f64, u8) = (230_i32, 11.9, 22_u8);

    let first = tup.0;
    let second = tup.1;
    let third = tup.2;

    println!("通过点访问  x ={} y ={} z ={}", first, second, third);

    let _array = [1, 2, 3, 4, 5];
    print_array(_array);

    let _months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    print_array(_months);

    let _array: [i32; 6] = [1, 2, 3, 4, 5, 6];
    print_array(_array);

    let _array = [0; 5];
    print_array(_array);
}

// 声明常量使用 const 关键字，并且 必须 注明值的类型
// 常量只能被设置为常量表达式，而不可以是其他任何只能在运行时计算出的值
const _THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

// fn print_array<T: std::fmt::Debug, const N: usize>(array: [T; N]) {
fn print_array<T, const N: usize>(array: [T; N])
where
    T: std::fmt::Debug,
{
    for i in array.iter() {
        print!("{:?} ", i);
    }
    println!();
}
