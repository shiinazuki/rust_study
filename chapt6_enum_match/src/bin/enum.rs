#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

// 使用枚举替换掉上面的 结构体和枚举的组合

#[derive(Debug)]
enum IpAddrNew {
    V4(String),
    V6(String),
}

#[derive(Debug)]
enum IpAddrOnther {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(six);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    println!("home = {:?}", home);
    println!("loopback = {:?}", loopback);

    let home = IpAddrNew::V4(String::from("127.0.0.1"));
    let loopback = IpAddrNew::V6(String::from("::1"));

    println!("home = {:?}", home);
    println!("loopback = {:?}", loopback);

    let home = IpAddrOnther::V4(127, 0, 0, 1);
    let loopback = IpAddrOnther::V6(String::from("::1"));

    println!("home = {:?}", home);
    println!("loopback = {:?}", loopback);

    let m = Message::Write(String::from("hello message"));
    m.call();

    

}

fn route(ip_kind: IpAddrKind) {
    println!("{:?}", ip_kind);
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}


impl Message {
    fn call(&self) {
        println!("{:?}", self)
    }
}
