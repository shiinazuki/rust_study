use std::fmt;

fn main() {
    // 摄氏度 华氏度 转换
    temp();

    // 生成第 n 个斐波那契数
    let n = 100;
    println!("第{}的斐波那契数是 {}", n, fibonacci(n));
}

// 生成第 n 个斐波那契数
fn fibonacci(n: usize) -> u128 {
    if n < 2 {
        return 1;
    }

    let mut a = 1;
    let mut b = 1;
    let mut c = 0;

    for _i in 2..n {
        c = a + b;
        a = b;
        b = c;
    }
    c

    /*     let mut vec = Vec::with_capacity(n);
    if n < 2 {
        return 1;
    }
    vec.push(1);
    vec.push(1);

    for i in 2..n {
        let fib = vec[i - 1] + vec[i - 2];
        vec.push(fib);
    }

    return vec[n - 1]; */
}

// 摄氏度 华氏度 转换
fn temp() {
    let temp = Temp::HuaShiDu(1000.0_f64);
    println!(
        "华氏度 {} 转摄氏度为 {:.2}",
        temp.clone(),
        sheshidu_and_huashidu(temp)
    );

    let temp = Temp::SheShiDu(1000.0_f64);
    println!(
        "摄氏度 {} 转华氏度为 {:.2}",
        temp.clone(),
        sheshidu_and_huashidu(temp)
    );
}

#[derive(Clone, Copy)]
enum Temp {
    SheShiDu(f64),
    HuaShiDu(f64),
}
impl fmt::Display for Temp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Temp::HuaShiDu(val) => write!(f, "{}", val),
            Temp::SheShiDu(val) => write!(f, "{}", val),
        }
    }
}

fn sheshidu_and_huashidu(temp: Temp) -> f64 {
    match temp {
        Temp::HuaShiDu(v) => return (v - 32 as f64) / 1.8,
        Temp::SheShiDu(v) => return 32 as f64 + v * 1.8,
    }
}
