fn main() {
    // 使用Box<T>指向堆上的数据
    // box 允许将一个值放在堆上而不是栈上。留在栈上的则是指向堆数据的指针

    // 应用场景
    /*
        当有一个在编译时未知大小的类型，而又想要在需要确切大小的上下文中使用这个类型值的时候
        当有大量数据并希望在确保数据不被拷贝的情况下转移所有权的时候
        当希望拥有一个值并只关心它的类型是否实现了特定 trait 而不是其具体类型的时候
    */

    // 使用 Box<T> 在堆上储存数据
    let b = Box::new(5);
    println!("b = {}", b);

    // Box 允许创建递归类型
    let list = List::Cons(
        1,
        Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
    );
    println!("{:?}", list);
}

#[derive(Debug)]
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}
