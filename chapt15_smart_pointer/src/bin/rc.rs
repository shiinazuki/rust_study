use std::rc::Rc;

fn main() {
    //  Rust 类型 Rc<T>，其为 引用计数（reference counting）的缩写

    // Rc<T> 只能用于单线程场景

    let a = Rc::new(Box::new(List::Cons(
        5,
        Rc::new(Box::new(List::Cons(10, Rc::new(Box::new(List::Nil))))),
    )));
    let b = List::Cons(3, Rc::clone(&a));
    let c = List::Cons(4, Rc::clone(&a));
    println!("b={:#?}", b);
    println!("c={:#?}", c);

    let a = Rc::new(List_Rc::Cons(
        5,
        Rc::new(List_Rc::Cons(10, Rc::new(List_Rc::Nil))),
    ));
    let b = List_Rc::Cons(3, Rc::clone(&a));
    let c = List_Rc::Cons(4, Rc::clone(&a));
    println!("b={:#?}", b);
    println!("c={:#?}", c);

    // 克隆 Rc<T> 会增加引用计数
    let a = Rc::new(List_Rc::Cons(
        5,
        Rc::new(List_Rc::Cons(10, Rc::new(List_Rc::Nil))),
    ));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = List_Rc::Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = List_Rc::Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));

    // 通过不可变引用， Rc<T> 允许在程序的多个部分之间只读地共享数据
    // 如果 Rc<T> 也允许多个可变引用，则会违反借用规则之一：相同位置的多个可变借用可能造成数据竞争和不一致
}

#[derive(Debug)]
enum List_Rc<T> {
    Cons(T, Rc<List_Rc<T>>),
    Nil,
}

#[derive(Debug)]
enum List<T> {
    Cons(T, Rc<Box<List<T>>>),
    Nil,
}
