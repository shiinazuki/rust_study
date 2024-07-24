fn main() {
    // 使用迭代器处理元素序列
    // 在 Rust 中，迭代器是 惰性的（lazy），这意味着在调用消费迭代器的方法之前不会执行任何操作

    let v1 = vec![1, 2, 3];
    let _v1_iter = v1.iter();

    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }

    // iter()       生成一个不可变引用的迭代器
    // iter_mut()   生成一个可变引用的迭代器
    // iter_into()  生成一个获取所有权的迭代器

    iterator_demonstration();

    // 消费迭代器    调用 next 方法的方法被称为 消费适配器（consuming adaptors）
    iterator_sum();

    // 产生其他迭代器的方法
    // Iterator trait 中定义了另一类方法，被称为 迭代器适配器（iterator adaptors）
    // 允许我们将当前迭代器变为不同类型的迭代器 可以链式调用多个迭代器适配器
    // 不过因为所有的迭代器都是惰性的，必须调用一个消费适配器方法以便获取迭代器适配器调用的结果

    let v1 = vec![1, 2, 3];

    let v1 = v1.iter().map(|x| x + 1);
    assert_eq!(vec![2, 3, 4], v1.collect::<Vec<_>>());



    
}

fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);

    // 调用 sum 之后不再允许使用 v1_iter 因为调用 sum 时它会获取迭代器的所有权
    // println!("{}", v1_iter.next());

    assert_eq!(v1.iter().sum::<i32>(), 6);

    // 使用捕获其环境的闭包
    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];

    let in_my_size = shoe_in_size(shoes, 10);
    assert_eq!(
        in_my_size,
        vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ]
    );
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: i32,
    style: String,
}

fn shoe_in_size(shoes: Vec<Shoe>, shoe_size: i32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}
