use std::{
    fs,
    io::{self, Error, ErrorKind},
    path::Path,
};

fn main() {
    // Result

    // is_ok() 如果Result为 OK，则返回 true。
    let x: Result<i32, &str> = Ok(-3);
    assert_eq!(x.is_ok(), true);

    let x: Result<i32, &str> = Err("Some error message");
    assert_eq!(x.is_ok(), false);

    // is_ok_and() 如果Result为 OK，且其中的值符合闭包，则返回 true
    let x: Result<i32, &str> = Ok(2);
    assert_eq!(x.is_ok_and(|x| x > 1), true);

    let x: Result<i32, &str> = Ok(0);
    assert_eq!(x.is_ok_and(|x| x > 1), false);

    let x: Result<u32, &str> = Err("hey");
    assert_eq!(x.is_ok_and(|x| x > 1), false);

    // is_err() 如果Result为 Err，则返回 true
    let x: Result<i32, &str> = Ok(-3);
    assert_eq!(x.is_err(), false);

    let x: Result<i32, &str> = Err("Some error message");
    assert_eq!(x.is_err(), true);

    // is_err_and() 如果Result为 Err 且其中的值符合闭包，则返回 true
    let x: Result<u32, Error> = Err(Error::new(ErrorKind::NotFound, "!"));
    assert_eq!(x.is_err_and(|x| x.kind() == ErrorKind::NotFound), true);

    let x: Result<u32, Error> = Err(Error::new(ErrorKind::PermissionDenied, "!"));
    assert_eq!(x.is_err_and(|x| x.kind() == ErrorKind::NotFound), false);

    let x: Result<u32, Error> = Ok(123);
    assert_eq!(x.is_err_and(|x| x.kind() == ErrorKind::NotFound), false);

    // ok() 从 Result<T, E> 转换为 Option<T>
    // 将 self 转换为 Option<T>，消耗 self 并丢弃错误（如果有）
    let x: Result<u32, &str> = Ok(2);
    assert_eq!(x.ok(), Some(2));

    let x: Result<u32, &str> = Err("Nothing here");
    assert_eq!(x.ok(), None);

    // err() 从 Result<T, E> 转换为 Option<E>
    // 将 self 转换为 Option<E>，消耗 self 并丢弃成功值（如果有的话)
    let x: Result<u32, &str> = Ok(2);
    assert_eq!(x.err(), None);

    let x: Result<u32, &str> = Err("Nothing here");
    assert_eq!(x.err(), Some("Nothing here"));

    // as_ref() 从 &Result<T, E> 转换为 Result<&T, &E>
    // 生成一个新result，其中包含对原始结果的引用，但保留原始结果
    let x: Result<u32, &str> = Ok(2);
    assert_eq!(x.as_ref(), Ok(&2));

    let x: Result<u32, &str> = Err("Error");
    assert_eq!(x.as_ref(), Err(&"Error"));

    // as_mut() 从 &mut Result<T, E> 转换为 Result<&mut T, &mut E>
    fn mutate(r: &mut Result<i32, i32>) {
        match r.as_mut() {
            Ok(v) => *v = 42,
            Err(e) => *e = 0,
        }
    }
    let mut x: Result<i32, i32> = Ok(2);
    mutate(&mut x);
    assert_eq!(x.unwrap(), 42);

    let mut x: Result<i32, i32> = Err(13);
    mutate(&mut x);
    assert_eq!(x.unwrap_err(), 0);

    // map() 通过对包含的 Ok 值应用函数，将 Result<T, E> 映射为 Result<U,E>，而不触及 Err 值
    // 该函数可用于合成两个函数的结果
    let line = "1\n2\n3\n4";
    for num in line.lines() {
        match num.parse::<i32>().map(|i| i * 2) {
            Ok(n) => println!("{}", n),
            Err(..) => {}
        }
    }

    // map_or() 返回提供的默认值（如果 Err），或对包含的值应用函数（如果 Ok）
    // 传递给 map_or 的参数是急于求值的；如果传递的是函数调用的结果，建议使用 map_or_else，它是懒于求值的
    let x: Result<_, &str> = Ok("foo");
    assert_eq!(x.map_or(42, |v| v.len()), 3);

    let x: Result<&str, _> = Err("bar");
    assert_eq!(x.map_or(42, |v| v.len()), 42);

    // map_or_else() 将回退函数 default 应用于包含的 Err 值，或将函数 f 应用于包含的 Ok 值，从而将 Result<T, E> 映射为 U
    // 该函数可用于在处理错误时解压缩成功的结果
    let k = 21;

    let x: Result<_, &str> = Ok("foo");
    assert_eq!(x.map_or_else(|_e| k * 2, |v| v.len()), 3);

    let x: Result<&str, _> = Err("bar");
    assert_eq!(x.map_or_else(|_e| k * 2, |v| v.len()), 42);

    // map_err() 通过对包含的 Err 值应用函数，将 Result<T, E> 映射到 Result<T, F> 中，而不触及 Ok 值
    // 该函数可用于在处理错误时传递成功结果
    fn stringify(x: u32) -> String {
        format!("error code: {x}")
    }
    let x: Result<u32, u32> = Ok(2);
    assert_eq!(x.map_err(stringify), Ok(2));

    let x: Result<u32, u32> = Err(13);
    assert_eq!(x.map_err(stringify), Err("error code: 13".to_string()));

    // inspect()  如果 OK，则调用指向所含值的引用的函数
    // 返回原始 Result
    let x: u8 = "4"
        .parse::<u8>()
        .inspect(|x| println!("original: {}", x))
        .map(|x| x.pow(3))
        .expect("failed to parse number");

    println!("{}", x);

    // inspect_err() 如果Err 调用函数
    fn _read() -> io::Result<String> {
        fs::read_to_string("README.md").inspect_err(|e| eprintln!("failed to read file: {}", e))
    }

    // as_deref() 从 Result<T, E>（或 &Result<T, E>）转换为 Result<&<T as Deref>::Target, &E>
    // 通过 Deref 强制原始结果的 Ok 变体，并返回新结果
    let x: Result<String, u32> = Ok("hello".to_string());
    let y: Result<&str, &u32> = Ok("hello");
    assert_eq!(x.as_deref(), y);

    let x: Result<String, u32> = Err(42);
    let y: Result<&str, &u32> = Err(&42);
    assert_eq!(x.as_deref(), y);

    // as_deref_mut() 从 Result<T, E> (或 &mut Result<T, E>) 转换为 Result<&mut <T as DerefMut>::Target, &mut E>
    let mut s = "HELLO".to_string();
    let mut x: Result<String, u32> = Ok("hello".to_string());
    let y: Result<&mut str, &mut u32> = Ok(&mut s);
    assert_eq!(
        x.as_deref_mut().map(|x| {
            x.make_ascii_uppercase();
            x
        }),
        y
    );

    let mut i = 42;
    let mut x: Result<String, u32> = Err(42);
    let y: Result<&mut str, &mut u32> = Err(&mut i);
    assert_eq!(
        x.as_deref_mut().map(|x| {
            x.make_ascii_uppercase();
            x
        }),
        y
    );

    // iter() 返回可能包含的值的迭代器
    let x: Result<u32, &str> = Ok(7);
    assert_eq!(x.iter().next(), Some(&7));

    let x: Result<u32, &str> = Err("nothing!");
    assert_eq!(x.iter().next(), None);

    // iter_mut() 返回可能包含的值的可变迭代器
    let mut x: Result<u32, &str> = Ok(7);
    match x.iter_mut().next() {
        Some(v) => *v = 40,
        None => {}
    }
    assert_eq!(x, Ok(40));

    let mut x: Result<u32, &str> = Err("nothing!");
    assert_eq!(x.iter_mut().next(), None);

    // expect() 返回包含的 Ok 值，消耗自身值
    // let x: Result<u32, &str> = Err("emergency failure");
    // x.expect("Testing expect");

    // unwrap() 返回包含的 Ok 值，消耗自身值
    // 由于该函数可能会引起恐慌，因此一般不建议使用
    // 相反，最好使用模式匹配并明确处理 Err 情况，或调用 unwrap_or、unwrap_or_else 或 unwrap_or_default。
    let x: Result<u32, &str> = Ok(2);
    assert_eq!(x.unwrap(), 2);

    // unwrap_or_default() 返回所包含的 Ok 值或默认值
    // 消耗 self 参数，如果 OK，返回包含的值，否则 Err，返回该类型的默认值
    let good_year_from_input = "1909";
    let bad_year_from_input = "190blarg";
    let good_year = good_year_from_input.parse::<i32>().unwrap_or_default();
    let bad_year = bad_year_from_input.parse::<i32>().unwrap_or_default();

    assert_eq!(1909, good_year);
    assert_eq!(0, bad_year);

    // expect_err() 返回包含的 Err 值，消耗自身值
    // 如果值是 Err 就返回Err值 否则报错
    // let x: Result<u32, &str> = Ok(10);
    // x.expect_err("Testing expect_err");

    // unwrap_err() 返回包含的 Err 值，消耗自身值
    // 如果值是 Err 就返回Err值 否则报错
    // let x: Result<u32, &str> = Ok(2);
    // x.unwrap_err();

    let x: Result<u32, &str> = Err("emergency failure");
    assert_eq!(x.unwrap_err(), "emergency failure");

    // and()  调用者是Ok 返回参数的值  调用者是Err 直接返回该 Err
    // 传递给 and 的参数是急于求值的；如果传递的是函数调用的结果，建议使用 and_then，它是懒于求值的
    // 调用者是Ok 返回参数的值  调用者是Err 直接返回该 Err
    let x: Result<u32, &str> = Ok(2);
    let y: Result<&str, &str> = Err("late error");
    assert_eq!(x.and(y), Err("late error"));

    let x: Result<u32, &str> = Err("early error");
    let y: Result<&str, &str> = Ok("foo");
    assert_eq!(x.and(y), Err("early error"));

    let x: Result<u32, &str> = Err("not a 2");
    let y: Result<&str, &str> = Err("late error");
    assert_eq!(x.and(y), Err("not a 2"));

    let x: Result<u32, &str> = Ok(2);
    let y: Result<&str, &str> = Ok("different result type");
    assert_eq!(x.and(y), Ok("different result type"));

    // and_then() 如果结果为 OK，则调用操作，否则返回 self 的 Err 值
    // 该函数可用于基于结果值的控制流
    fn sq_then_to_string(x: u32) -> Result<String, &'static str> {
        x.checked_mul(x)
            .map(|sq| sq.to_string())
            .ok_or("overflowed")
    }

    assert_eq!(Ok(2).and_then(sq_then_to_string), Ok(4.to_string()));
    assert_eq!(Ok(1_000_000).and_then(sq_then_to_string), Err("overflowed"));
    assert_eq!(
        Err("not a number").and_then(sq_then_to_string),
        Err("not a number")
    );

    // 通常用于连锁可能返回 Err 的错误操作
    let root_modified_time = Path::new("/").metadata().and_then(|md| md.modified());
    assert!(root_modified_time.is_ok());

    let should_fail = Path::new("/bad/path")
        .metadata()
        .and_then(|md| md.modified());
    assert!(should_fail.is_err());
    assert_eq!(should_fail.unwrap_err().kind(), ErrorKind::NotFound);

    // or() 如果调用者值是Ok 返回调用者的值  否则返回参数的值
    // 传递给 or 的参数是急于求值的；如果传递的是函数调用的结果，建议使用 or_else，它是懒于求值的
    let x: Result<u32, &str> = Ok(2);
    let y: Result<u32, &str> = Err("late error");
    assert_eq!(x.or(y), Ok(2));

    let x: Result<u32, &str> = Err("early error");
    let y: Result<u32, &str> = Ok(2);
    assert_eq!(x.or(y), Ok(2));

    let x: Result<u32, &str> = Err("not a 2");
    let y: Result<u32, &str> = Err("late error");
    assert_eq!(x.or(y), Err("late error"));

    let x: Result<u32, &str> = Ok(2);
    let y: Result<u32, &str> = Ok(100);
    assert_eq!(x.or(y), Ok(2));

    // or_else() 如果结果是 Err，则调用操作，否则返回 self 的 Ok 值
    // 该函数可用于基于结果值的控制流
    fn sq(x: u32) -> Result<u32, u32> {
        Ok(x * x)
    }

    fn err(x: u32) -> Result<u32, u32> {
        Err(x)
    }

    assert_eq!(Ok(2).or_else(sq).or_else(sq), Ok(2));
    assert_eq!(Ok(2).or_else(err).or_else(sq), Ok(2));
    assert_eq!(Err(3).or_else(sq).or_else(err), Ok(9));
    assert_eq!(Err(3).or_else(err).or_else(err), Err(3));

    // unwrap_or() 返回包含的 Ok 值或提供的默认值
    // 传递给 unwrap_or 的参数是急于求值的；如果传递的是函数调用的结果，建议使用 unwrap_or_else，它是懒于求值的
    let default = 2;
    let x: Result<u32, &str> = Ok(9);
    assert_eq!(x.unwrap_or(default), 9);

    let x: Result<u32, &str> = Err("error");
    assert_eq!(x.unwrap_or(default), default);

    // unwrap_or_else() 返回所包含的 Ok 值或通过闭包计算 Ok 值
    assert_eq!(Ok(2).unwrap_or_else(|x| x), 2);
    assert_eq!(Err("foo").unwrap_or_else(|x| x.len()), 3);

    // unwrap_unchecked() 返回包含的 Ok 值，消耗 self 值，不检查该值是否为 Err
    let x: Result<u32, &str> = Ok(2);
    assert_eq!(unsafe { x.unwrap_unchecked() }, 2);

    // let x: Result<u32, &str> = Err("emergency failure");
    // unsafe {
    //     x.unwrap_unchecked();
    // }

    // unwrap_err_unchecked() 返回包含的 Err 值，消耗自身值，且不检查该值是否为 Ok
    // let x: Result<u32, &str> = Ok(2);
    // unsafe { x.unwrap_err_unchecked() };

    let x: Result<u32, &str> = Err("emergency failure");
    assert_eq!(unsafe { x.unwrap_err_unchecked() }, "emergency failure");

    // copied() copied()   适用于 Result<&T, &E> 类型，其中 T 和 E 实现了 Copy trait。
    // 将引用类型转换为值类型
    // 速度较快，因为 Copy 只进行内存复制
    let val = 12;
    let x: Result<&i32, i32> = Ok(&val);
    assert_eq!(x, Ok(&12));
    let copied = x.copied();
    assert_eq!(copied, Ok(12));

    // 通过复制 Ok 部分的内容，将结果<&mut T, E>映射为结果<T, E>
    let mut val = 12;
    let x: Result<&mut i32, i32> = Ok(&mut val);
    assert_eq!(x, Ok(&mut 12));
    let copied = x.copied();
    assert_eq!(copied, Ok(12));

    // cloned() 通过克隆 Ok 部分的内容，将结果<&T, E>映射为结果<T, E>
    let val = 12;
    let x: Result<&i32, i32> = Ok(&val);
    assert_eq!(x, Ok(&12));
    let cloned = x.cloned();
    assert_eq!(cloned, Ok(12));

    // 通过克隆 Ok 部分的内容，将结果<&mut T, E>映射为结果<T, E>
    let mut val = 12;
    let x: Result<&mut i32, i32> = Ok(&mut val);
    assert_eq!(x, Ok(&mut 12));
    let cloned = x.cloned();
    assert_eq!(cloned, Ok(12));

    // transpose() 将一个选项的结果转换成一个结果的选项
    // Ok(None) 将被映射为 None。 Ok(Some(_)) 和 Err(_) 将被映射为 Some(Ok(_)) 和 Some(Err(_))
    let x: Result<Option<i32>, SomeErr> = Ok(Some(5));
    let y: Option<Result<i32, SomeErr>> = Some(Ok(5));
    assert_eq!(x.transpose(), y);
}

#[derive(Debug, Eq, PartialEq)]
struct SomeErr;
