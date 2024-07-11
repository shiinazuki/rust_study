pub fn study_vec() {
    let v: Vec<i32> = Vec::new();

    // 使用 vec! 宏 来创建一个 vector
    let v = vec![1, 2, 3];
    for i in v.iter() {
        println!("{}", i);
    }

    // 更新 vector
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // 读取 vector 的元素 通过索引或使用 get 方法

    let mut v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

    let first = &v[0];
    println!("The first element is {}", first);

    let third = v.get(2);
    match third {
        Some(third) => println!("The first element is {}", third),
        None => println!("There is no third element."),
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];

    let first = &v[0];

    // 已经存在一个不可变引用 不能 存在可变引用
    // v.push(6);

    println!("The first element is {}", first);
    // 在 vector 的结尾增加新元素时，在没有足够空间将所有元素依次相邻存放的情况下，
    // 可能会要求分配新内存并将老的元素拷贝到新的空间中。
    // 这时，第一个元素的引用就指向了被释放的内存。借用规则阻止程序陷入这种状况

    // 遍历 vector 中的元素
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }
    println!("========================");
    let mut v = vec![100, 33, 45];
    for i in &mut v {
        *i += 88;
        println!("{}", i);
    }
}

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

// 使用枚举来储存多种类型
pub fn enum_more() {
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.32),
    ];

    println!("{:?}", row);
}
