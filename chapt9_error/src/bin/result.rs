use std::{
    error::Error,
    fs::{self, File},
    io::{self, ErrorKind, Read},
    net::IpAddr,
};

fn main() -> Result<(), Box<dyn Error>> {
    // 打开一个文件
    let greeting_file_result = File::open("README.md");
    println!("{:?}", greeting_file_result);
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
    println!("{:?}", greeting_file);

    // 匹配不同的错误
    let greeting_file_result = File::open("README.md");
    let _greeting_flie = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            _other_error => {
                panic!("Problem opening the file: {:?}", error);
            }
        },
    };

    let _greeting_file = File::open("README.md").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("README.md").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // 失败时 panic 的简写：unwrap 和 expect
    let _greeting_file = File::open("README.md").unwrap();
    let _greeting_file =
        File::open("README.md").expect("README.md should be included in this project");

    // 传播错误
    let username = read_username_from_file();
    let username = match username {
        Ok(file) => file,
        Err(error) => panic!("xxxxx{}", error),
    };

    println!("{}", username);

    // 传播错误的简写：? 运算符

    // 哪里可以使用 ? 运算符

    println!("{}", last_char_of_first_line(&"text").unwrap());

    // 可以在返回 Result 的函数中对 Result 使用 ? 运算符，可以在返回 Option 的函数中对 Option 使用 ? 运算符，
    // 但是不可以混合搭配
    // 使用类似 Result 的 ok 方法或者 Option 的 ok_or 方法来显式转换
    let _greeting_file = File::open("README.md")?;

    let _home: IpAddr = "127.0.0.1"
        .parse()
        .expect("Hardcoded IP address should be valid");

    Ok(())
}

// 从给定文本中返回第一行最后一个字符
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

// 从文件中读取数据的函数。如果文件不存在或不能读取，这个函数会将这些错误返回给调用它的代码
fn read_username_from_file() -> Result<String, io::Error> {
    let greeting_file_res = File::open("README.md");

    let mut greeting_file = match greeting_file_res {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut res_str = String::new();
    let _ = match greeting_file.read_to_string(&mut res_str) {
        Ok(_) => Ok(res_str),
        Err(e) => Err(e),
    };

    // 下面两行 等于上面的写法
    let mut str = String::new();
    File::open("README.md")?.read_to_string(&mut str)?;

    // rust 提供了一个函数 直接调用即可实现同样功能
    //  Rust 提供了名为 fs::read_to_string 的函数，它会打开文件、新建一个 String、读取文件的内容，并将内容放入 String，接着返回它
    fs::read_to_string("README.md")
}
