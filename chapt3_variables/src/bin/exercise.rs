use std::fmt;

fn main() {
    // 摄氏度 华氏度 转换
    temp();

    // 生成第 n 个斐波那契数
    let n = 100;
    println!("第{}的斐波那契数是 {}", n, fibonacci(n));

    print();
}

// 打印圣诞颂歌
fn print() {
    let week = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    let song: [String; 12] = [
        "a partridge in a pear tree.".to_owned(),
        "Two turtle doves,".to_owned(),
        "Three French hens,".to_owned(),
        "Four calling birds,".to_owned(),
        "Five golden rings,".to_owned(),
        "Six geese a-laying,".to_owned(),
        "Seven swans a-swimming,".to_owned(),
        "Eight maids a-milking,".to_owned(),
        "Nine ladies dancing,".to_owned(),
        "Ten lords a-leaping,".to_owned(),
        "Eleven pipers piping,".to_owned(),
        "Twelve drummers drumming,".to_owned(),
    ];

    for (index, context) in song.iter().enumerate() {
        println!("On the {} day of Christmas,", week[index]);
        println!("my true love gave to me");
        if index == 0 {
            println!("A partridge in a pear tree.");
        } else {
            println!("{}", context);
            for j in (0..index).rev() {
                if j == 0 && j != 0 {
                    println!("And");
                }
                println!("{}", song[j]);
            }
        }
        println!();
    }
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
