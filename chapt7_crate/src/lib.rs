mod front_of_house;
mod back_of_house;

// 通过 use 引入作用域的路径也会检查私有性，同其它路径一样
use crate::front_of_house::hosting;
use std::collections::HashMap;

use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    todo!()
}

fn function2() -> IoResult<()> {
    todo!()
}

pub fn eat_at_restaurant() {
    // 在夏天订购一个黑麦土司作为早餐
    let mut meal = back_of_house::Breakfase::summer("Rye");
    // 改变主意更换想要面包的类型
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // 如果取消下一行的注释代码不能编译；
    // 不允许查看或修改早餐附带的季节水果
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    // 使用use 简化路径
    hosting::add_to_waitlist();

    let mut map = HashMap::new();
    map.insert(1, 2);
    println!("{:?}", map.get(&1));
}

fn deliver_order() {}

