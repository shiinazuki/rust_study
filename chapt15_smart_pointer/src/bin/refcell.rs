use std::{cell::RefCell, rc::Rc};

fn main() {
    // RefCell<T> 和内部可变性模式
    // 内部可变性（Interior mutability）是 Rust 中的一个设计模式
    // 它允许你即使在有不可变引用时也可以改变数据，这通常是借用规则所不允许的

    // 通过 RefCell<T> 在运行时检查借用规则
    // RefCell<T> 代表其数据的唯一的所有权

    // 借用规则：
    // 在任意给定时刻，只能拥有一个可变引用或任意数量的不可变引用 之一（而不是两者）
    // 引用必须总是有效的

    // 对于引用和 Box<T>，借用规则的不可变性作用于编译时。对于 RefCell<T>，这些不可变性作用于 运行时
    // 对于引用，如果违反这些规则，会得到一个编译错误。而对于 RefCell<T>，如果违反这些规则程序会 panic 并退出

    // RefCell<T> 用于当你确信代码遵守借用规则，而编译器不能理解和确定的时候

    // RefCell<T> 只能用于单线程场景

    // !!!!!
    /*
        选择 Box<T>，Rc<T> 或 RefCell<T> 的理由：
            Rc<T> 允许相同数据有多个所有者  Box<T> 和 RefCell<T> 有单一所有者
            Box<T> 允许在编译时执行不可变或可变借用检查；Rc<T>仅允许在编译时执行不可变借用检查；RefCell<T> 允许在运行时执行不可变或可变借用检查
            因为 RefCell<T> 允许在运行时执行可变借用检查，所以我们可以在即便 RefCell<T> 自身是不可变的情况下修改其内部的值
    */

    // 在不可变值内部改变值就是 内部可变性 模式

    // 内部可变性：不可变值的可变借用
    // 借用规则的一个推论是 当有一个不可变值时，不能可变地借用它
    let _x = 5;
    // let y = &mut x;

    // 内部可变性的用例：mock 对象

    let user = User {
        username: String::from("shiina"),
        age: 18,
    };
    let mut limit = LimitTracker::new(&user, 20);
    limit.set_value(17);

    // 结合 Rc<T> 和 RefCell<T> 来拥有多个可变数据所有者

    // Rc<T> 允许对相同数据有多个所有者，不过只能提供数据的不可变访问。
    // 如果有一个储存了 RefCell<T> 的 Rc<T> 的话，就可以得到有多个所有者 并且 可以修改的值了

    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(List::Cons(Rc::clone(&value), Rc::new(List::Nil)));

    let b = List::Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = List::Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);

    // RefCell<T> 不能用于多线程代码！Mutex<T> 是一个线程安全版本的 RefCell<T>
}

#[derive(Debug)]
enum List<T> {
    Cons(T, Rc<List<T>>),
    Nil,
}

trait Messenger {
    fn send(&self, msg: &str);
}

struct User {
    username: String,
    age: usize,
}

impl Messenger for User {
    fn send(&self, msg: &str) {
        println!("{}", msg);
    }
}

struct LimitTracker<'a, T: Messenger> {
    messanger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    fn new(messanger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messanger,
            value: 0,
            max,
        }
    }

    fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messanger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messanger
                .send("urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max > 0.75 {
            self.messanger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

// borrow 方法返回 Ref<T> 类型的智能指针，borrow_mut 方法返回 RefMut<T> 类型的智能指针。这两个类型都实现了 Deref

// RefCell<T> 记录当前有多少个活动的 Ref<T> 和 RefMut<T> 智能指针
// 每次调用 borrow，RefCell<T> 将活动的不可变借用计数加一当 Ref<T> 值离开作用域时，不可变借用计数减一
// !!!!!
// 就像编译时借用规则一样，RefCell<T> 在任何时候只允许有多个不可变借用或一个可变借用

#[cfg(test)]
mod test {
    use std::cell::RefCell;

    use super::*;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }
    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            self.sent_messages.borrow_mut().push(String::from(msg));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
