use std::{sync::mpsc, thread, time::Duration};

fn main() {
    // 使用消息传递在线程间传送数据

    // mpsc 是 多个生产者，单个消费者（multiple producer, single consumer）的缩写
    let (tx, rx) = mpsc::channel();

    let handle1 = thread::spawn(move || {
        let message = "hello main thread";
        tx.send(message).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);

    handle1.join().unwrap();

    // 信道与所有权转移

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hello main thread");
        tx.send(val).unwrap();
        // 消息已经被发送给接受者  后续不可在使用
        // println!("val is {}", val);
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);

    // 发送多个值并观察接收者的等待
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }

    println!("======================");
    
    // 通过克隆发送者来创建多个生产者
    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }


}
