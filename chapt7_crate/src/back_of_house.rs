// 结构体 公有 与字段无关
// 即使结构体被设置为共有  字段没有设置的话还是私有
pub struct Breakfase {
    pub toast: String,
    seasonal_fruit: String,
}

impl Breakfase {
    pub fn summer(toast: &str) -> Breakfase {
        Breakfase {
            toast: String::from(toast),
            seasonal_fruit: String::from("peaches"),
        }
    }
}

// 将枚举设为公有，则它的所有成员都将变为公有
pub enum Appetizer {
    Soup,
    Salad,
}

fn fix_incorrect_order() {
    cook_order();
    super::deliver_order();
}

fn cook_order() {}