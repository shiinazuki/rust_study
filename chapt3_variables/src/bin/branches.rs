fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let condition = true;
    // if 的每个分支的可能的返回值都必须是相同类型
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);

    let _condition = true;
    // 类型不匹配  编译错误
    // let number = if condition { 5 } else { "six" };

    let mut count = 0;
    loop {
        count += 1;
        if count == 100 {
            break;
        };
        print!("{} ", count);
    }

    println!();

    // 从循环返回值

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is  {}", result);

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    let mut number = 3;

    while number != 0 {
        println!("{number}");
        number -= 1;
    }

    println!("LIFTOFF");

    // while 遍历数据
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < a.len() {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    println!();
    // 使用for循环遍历数组
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {}", element);
    }

    // Range
    for number in (1..=4).rev() {
        println!("{}", number);
    }

    println!("LIFTOFF");
    
}
