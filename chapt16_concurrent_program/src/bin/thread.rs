use std::{thread, time::Duration};

fn main() {
    // 并发编程（Concurrent programming）代表程序的不同部分相互独立地执行
    // 并行编程（parallel programming）代表程序不同部分同时执行

    // 如下是本章将要涉及到的内容
    /*
        如何创建线程来同时运行多段代码
        消息传递（Message passing）并发，其中信道（channel）被用来在线程间传递消息
        共享状态（Shared state）并发，其中多个线程可以访问同一片数据
        Sync 和 Send trait，将 Rust 的并发保证扩展到用户定义的以及标准库提供的类型中
    */

    // 已执行程序的代码在一个 进程（process）中运行，操作系统则会负责管理多个进程
    // 在程序内部，也可以拥有多个同时运行的独立部分。这些运行这些独立部分的功能被称为 线程

    // 竞态条件（Race conditions），多个线程以不一致的顺序访问数据或资源
    // 死锁（Deadlocks），两个线程相互等待对方，这会阻止两者继续运行
    // 只会发生在特定情况且难以稳定重现和修复的 bug

    // 使用线程同时运行代码
    // 使用 spawn 创建新线程
    // 使用 join 等待所有线程结束
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // 在这里调用join的话 会阻塞main线程  这并没有发挥出多线程的优势  甚至不如一个main线程执行的快
    //  handle.join().unwrap();

    // !!!!!
    // 当 Rust 程序的主线程结束时，新线程也会结束，而不管其是否执行完毕
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // JoinHandle 是一个拥有所有权的值，当对其调用 join 方法时，它会等待其线程结束
    handle.join().unwrap();

    // 通过调用 handle 的 join 会阻塞当前线程直到 handle 所代表的线程结束。
    // 阻塞（Blocking）线程意味着阻止该线程执行工作或退出

    // 将 move 闭包与线程一同使用

    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("spawned thread Here's a vector: {:?}", v);
    });
    // Rust 会 推断 如何捕获 v，因为 println! 只需要 v 的引用，闭包尝试借用 v
    // 然而这有一个问题：Rust 不知道这个新建线程会执行多久，所以无法知晓对 v 的引用是否一直有效

    // 这样是不行的
    // drop(v);

    handle.join().unwrap();

    // move 关键字覆盖了 Rust 默认保守的借用，但它不允许我们违反所有权规则
}
