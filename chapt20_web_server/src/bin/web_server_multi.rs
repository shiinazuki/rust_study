use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use chapt20_web_server::ThreadPool;

//将单线程 server 变为多线程 server
// 使用线程池改善吞吐量

const HOST: &str = "127.0.0.1:7878";
const THREAD_SIZE: usize = 4;

fn main() {
    // 流（stream）代表一个客户端和服务端之间打开的连接
    // 连接（connection）代表客户端连接服务端、服务端生成响应以及服务端关闭连接的全部请求 / 响应过程

    // 监听 TCP 连接
    let listener = TcpListener::bind(HOST).unwrap();

    // 创建容量为4的线程池
    let pool = ThreadPool::new(THREAD_SIZE);

    for stream in listener.incoming()/*.take(2)*/ {
        let stream = stream.unwrap();
        println!("Connection established!");
        // println!("{:?}", stream);

        // 为每一个请求分配线程 这种方式可以 但是不合适
        // thread::spawn(|| {
        // 读取请求
        // handle_connection(stream);
        // });

        // 创建有限数量的线程
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

// 处理请求方法
fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    // let http_request = buf_reader
    //     .lines()
    //     .map(|result| result.unwrap())
    //     .take_while(|line| !line.is_empty())
    //     .collect::<Vec<_>>();

    // println!("Request: {:#?}", http_request);

    let request_line = buf_reader.lines().next().unwrap().unwrap();

    // 验证请求并有选择的进行响应
    // 编写响应
    // let (status_line, filename) = if buf_reader.lines().next().unwrap().unwrap() == "GET / HTTP/1.1"
    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", ".\\chapt20_web_server\\hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", ".\\chapt20_web_server\\hello.html")
        }

        _ => ("HTTP/1.1 404 NOT FOUND", ".\\chapt20_web_server\\404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line, length, contents
    );

    stream.write_all(response.as_bytes()).unwrap();
}

// request
// Method Request-URI HTTP-Version CRLF
// headers CRLF
// message-body

// response
// HTTP-Version Status-Code Reason-Phrase CRLF
// headers CRLF
// message-body
