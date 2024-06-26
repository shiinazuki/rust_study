// match 控制流结构
// 模式可由字面值、变量、通配符和许多其他内容构成

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

fn main() {
    let dime = value_in_cents(Coin::Dime);
    println!("{}", dime);

    let quarter = value_in_cents(Coin::Quarter(UsState::Alaska));
    println!("{}", quarter);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{:?}", six);
    println!("{:?}", none);

    // 匹配是穷尽的 分支必须覆盖了所有的可能性
    let dice_roll = 9;
    match dice_roll {
        3 => println!("获得神奇帽子  不会移动"),
        7 => println!("失去神奇帽子"),
        _ => println!("移动{}步", dice_roll),
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(x) => Some(x + 1),
        None => None,
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => {
            println!("Licky Dime");
            10
        }
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        }
    }
}
