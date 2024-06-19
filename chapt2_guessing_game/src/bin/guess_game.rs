use std::{
    cmp::Ordering,
    io::{self},
};

use rand::{self, Rng};

fn main() {
    guess_game();
}

fn guess_game() {
    println!("guess game start");

    // 使用rand crate 生成 1~100 的随机数
    let random_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("input a number");

        let mut guess = String::new();
        // 使用 stdin 来接收用户输入
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // 将输入的字符串 转为 u32类型
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // 比较
        match guess.cmp(&random_number) {
            Ordering::Greater => println!("太大了"),
            Ordering::Less => println!("太小了"),
            Ordering::Equal => {
                println!("恭喜猜对");
                break;
            }
        }
    }
}
