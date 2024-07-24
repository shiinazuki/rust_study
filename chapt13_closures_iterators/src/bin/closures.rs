use std::{cmp::Reverse, thread, time::Duration};

fn main() {
    // Rust 的 闭包（closures）是可以保存在一个变量中或作为参数传递给其他函数的匿名函数

    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(0));
        num
    };

    println!("{}", expensive_closure(2));

    let example_closure = |x| x;
    let s = example_closure(String::from("hello closure"));
    // 因为已经是字符串类型了 所以 在传整数 就报错了
    // let n = example_closure(5);

    println!("s={}", s);

    // 捕获引用或者移动所有权

    // 闭包可以通过三种方式捕获其环境    不可变借用，可变借用和获取所有权

    // 不可变借用
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let only_borrows = || println!("From closure: {:?}", list);

    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);

    // 可变借用
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrow_mutably = || list.push(39);

    borrow_mutably();

    println!("After calling closure: {:?}", list);

    // 获得所有权
    // 强制闭包获取它用到的环境中值的所有权，可以在参数列表前使用 move 关键字
    let list = vec![1, 2, 3];
    println!("Brfore defining closure: {:?}", list);

    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();

    // 将被捕获的值移出闭包和 Fn trait
    // 闭包体可以做以下任何事：将一个捕获的值移出闭包，修改捕获的值，既不移动也不修改值，或者一开始就不从环境中捕获值

    /*
        取决于闭包体如何处理值，闭包自动、渐进地实现一个、两个或三个 Fn trait。

        1. FnOnce 适用于能被调用一次的闭包，所有闭包都至少实现了这个 trait，因为所有闭包都能被调用。
            一个会将捕获的值移入闭包体的闭包只实现 FnOnce trait，这是因为它只能被调用一次。
        2. FnMut 适用于不会将捕获的值移入闭包体的闭包，但它可能会修改被捕获的值。这类闭包可以被调用多次。

        3. Fn 适用于既不将被捕获的值移入闭包体也不修改被捕获的值的闭包，当然也包括不从环境中捕获值的闭包。
            这类闭包可以被调用多次而不改变它们的环境，这在会多次并发调用闭包的场景中十分重要。
    */

    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    list.sort_by_key(|r| r.width);
    println!("{:#?}", list);
    println!("===========================================");
    
    
     // 使用Reverse 来反转属性
    list.sort_by_key(|r| Reverse(r.width));
    println!("{:#?}", list);
    println!("===========================================");

    let mut num_sort_operations = 0;
    list.sort_by_key(|r|  {
        num_sort_operations += 1;
        r.width
    });
    println!("{:#?}, sorted in {} operations", list, num_sort_operations);
    
    println!("===========================================");
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        // 统计各种颜色的个数
        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                &ShirtColor::Blue => num_blue += 1,
            }
        }
        // 找到最多的返回
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}
