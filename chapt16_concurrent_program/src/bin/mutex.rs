use std::{
    sync::{Arc, Mutex},
    thread,
};

fn main() {
    // 共享状态并发

    // 互斥器一次只允许一个线程访问数据 mutex

    /*
         互斥器以难以使用著称，因为你不得不记住：
            在使用数据之前尝试获取锁。
            处理完被互斥器所保护的数据之后，必须解锁数据，这样其他线程才能够获取锁
    */

    let m = Mutex::new(5);
    {
        // 使用 lock 方法获取锁，以访问互斥器中的数据。这个调用会阻塞当前线程，直到我们拥有锁为止
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    println!("m = {:?}", m);

    // 如果另一个线程拥有锁，并且那个线程 panic 了，则 lock 调用会失败
    // 在这种情况下，没人能够再获取锁，所以这里选择 unwrap 并在遇到这种情况时使线程 panic

    // !!!!!
    // 在线程间共享 Mutex<T>

    // counter 值在上一次循环中被移动了  所以这样写传入到线程不对
    // let counter = Mutex::new(0);

    // Rc<T> 并不能安全的在线程间共享 所以这样写也不行
    // let counter = Rc::new(Mutex::new(0));

    // 原子引用计数 Arc<T>
    let counter = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());

    // RefCell<T>/Rc<T>  会出现引用循环问题
    // Mutex<T>/Arc<T>   会出现 死锁问题
}
