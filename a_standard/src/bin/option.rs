#![feature(option_get_or_insert_default)]

use std::{clone, env, fs};

/** Option 枚举 方法  */

fn main() {
    // is_some()方法 如果 Option 有值，则返回 true 否则返回 false
    let x: Option<u32> = Some(2);
    let x = x.is_some();
    assert_eq!(x, true);

    let x: Option<u32> = None;
    let x = x.is_some();
    assert_eq!(x, false);

    // is_some_and() 接收一个闭包
    // 如果 满足闭包中的条件  返回 true  否则 返回 false
    let x = Some(2);
    let x = x.is_some_and(|x| x > 1);
    assert_eq!(x, true);

    let x = Some(0);
    let x = x.is_some_and(|x| x > 1);
    assert_eq!(x, false);

    let x: Option<u32> = None;
    let x = x.is_some_and(|x| x > 1);
    assert_eq!(x, false);

    //  is_none() 如果 Option 是None 返回 true  否则返回 false
    let x = Some(2);
    let x = x.is_none();
    assert_eq!(x, false);

    let x: Option<u32> = None;
    let x = x.is_none();
    assert_eq!(x, true);

    // as_ref()  从 &Option<T> 转换为 Option<&T> 理解是 把 内容 变为不可变引用
    let text = Some("Hello, world!".to_string());
    // 首先，使用 `as_ref` 将 `Option<String>` 转换为 `Option<&String>`、
    // 然后用 `map` 读取 *that* ，把 `text` 留在堆栈上。
    let text_length = text.as_ref().map(|s| s.len());
    println!("still can print text: {:?}", text);

    // as_mut() Converts from &mut Option<T> to Option<&mut T> 把内容 变为可变引用
    let mut x = Some(2);
    match x.as_mut() {
        Some(v) => *v = 43,
        None => {}
    }
    assert_eq!(x, Some(43));

    // as_pin_ref Converts from Pin<&Option<T>> to Option<Pin<&T>>. 不移动的不可变引用

    // as_pin_mut Converts from Pin<&mut Option<T>> to Option<Pin<&mut T>> 不移动的可变引用

    //  as_slice  返回所含值的片段 如果有值 如果为None，则返回一个空切片
    let x = Some(1234).as_slice();
    println!("{:?}", x); // [1234]

    //  as_mut_slice 返回所含值的可变片段 如果有值。如果为None，则返回一个空切片
    let mut x = Some(1234);
    x.as_mut_slice()[0] += 10;
    println!("{:?}", x);

    // expect() 返回包含的 Some 值，消耗自身值。
    // 如果值是 None，并带有 msg 提供的自定义 panic 信息，则会panic
    let x = Some("value");
    // let x: Option<_> = None;
    let x = x.expect("fruits are healthy");
    println!("{}", x);
    assert_eq!(x, "value");
    // 推荐信息风格
    let x = Some(1234).as_slice();
    let item = x.get(0).expect("slice should not be empty");
    println!("{}", item);

    //  unwrap() 返回包含的 Some 值，消耗自身值。 如果自身值等于 "None"，则会panic
    // 由于该函数可能会引起恐慌，因此一般不建议使用。相反，最好使用模式匹配并明确处理 None 情况，
    // 或调用 unwrap_or、unwrap_or_else 或 unwrap_or_default。
    let x = Some("air");
    assert_eq!(x.unwrap(), "air");

    // let x: Option<&str> = None;
    // assert_eq!(x.unwrap(), "air"); // fails

    // // unwrap_or()  返回包含的Some值 或 提供的默认值。
    // 传递给 unwrap_or 的参数是急于求值的；如果传递的是函数调用的结果，建议使用 unwrap_or_else，它是懒于求值的。
    let x = Some("car");
    let x = x.unwrap_or("bike");
    println!("{}", x);
    assert_eq!(x, "car");

    let x: Option<&str> = None;
    let x = x.unwrap_or("bike");
    println!("{}", x);
    assert_eq!(x, "bike");

    // unwrap_or_else  返回包含的Some值，或通过闭包计算该值
    let k = 10;
    let x = Some(4);
    let x = x.unwrap_or_else(|| 2 * k);
    println!("{}", x);
    assert_eq!(x, 4);

    let x: Option<i32> = None;
    let x = x.unwrap_or_else(|| 2 * k);
    println!("{}", x);
    assert_eq!(x, 20);

    // unwrap_or_default 返回包含的Some值 或 默认值。
    // 包含 self 参数，如果是 Some，则返回包含的值，否则如果是 None，则返回该类型的默认值
    let x = Some(12);
    let x = x.unwrap_or_default();
    println!("{}", x);
    assert_eq!(x, 12);

    let x: Option<i32> = None;
    let x = x.unwrap_or_default();
    println!("{}", x);
    assert_eq!(x, 0);

    // unwrap_unchecked  返回包含的 Some值，消耗自身值，但不检查该值是否为None。
    // 安全的  对 None 调用此方法是未定义的行为
    let x = Some("air");
    let x = unsafe { x.unwrap_unchecked() };
    println!("{}", x);
    assert_eq!(x, "air");

    // let x: Option<&str> = None;
    // let x = unsafe { x.unwrap_unchecked() };
    // println!("{}", x);
    // assert_eq!(x, "air");

    // inspect() 接受一个闭包，当 Option 包含 Some 值时，会调用这个闭包，并将值的引用传递给它。
    // 这个方法返回的是原始的 Option，可以继续进行链式调用
    let list = vec![1, 2, 3];
    let x = list
        .get(1)
        .inspect(|x| println!("got: {}", x))
        .unwrap_or(&0);
    println!("{}", x);
    // 获取到的是 None 就不调用闭包
    let x = list.get(5);
    println!("{:?}", x);
    x.inspect(|x| println!("got:{}", x));

    // map() 通过将一个函数应用到一个包含的值（如果是 Some）
    // 将 Option<T> 映射到 Option<U>，或者返回 None（如果是 None）。
    // 实例: 以 Option<usize> 的形式计算 Option<String> 的长度，并消耗原始长度
    let maybe_some_string = Some(String::from("Hello, World"));
    let maybe_some_len = maybe_some_string.map(|s| s.len());
    println!("{:?}", maybe_some_len);
    assert_eq!(maybe_some_len, Some(12));

    let x: Option<&str> = None;
    let x = x.map(|s| s.len());
    println!("{:?}", x);
    assert_eq!(x, None);

    // map_or() 返回提供的默认结果（if none），或对包含的值（如果有值）应用一个函数。
    // 传递给 map_or 的参数是急于求值的；如果传递的是函数调用的结果，建议使用 map_or_else，它是懒于求值的。
    let x = Some("foo");
    let x = x.map_or(55, |v| v.len());
    println!("{}", x);
    assert_eq!(x, 3);

    let x: Option<&str> = None;
    let x = x.map_or(50, |v| v.len());
    println!("{}", x);
    assert_eq!(x, 50);

    // map_or_else() 计算默认函数结果（if none），或对所含值应用不同的函数（如果有）。
    let k = 31;

    let x = Some("foo");
    let x = x.map_or_else(|| 2 * k, |v| v.len());
    println!("{}", x);
    assert_eq!(x, 3);

    let x: Option<&str> = None;
    let x = x.map_or_else(|| 2 * k, |v| v.len());
    println!("{}", x);
    assert_eq!(x, 62);

    // ok_or() 将 Option<T> 转换为 Result<T,E>，将 Some(v) 映射为 Ok(v)，将 None 映射为 Err(err)。
    // 传递给 ok_or 的参数是急于求值的；如果传递的是函数调用的结果，建议使用 ok_or_else，它是懒于求值的。
    let x = Some("foo");
    let x = x.ok_or(0);
    println!("{:?}", x);
    assert_eq!(x, Ok("foo"));

    let x: Option<&str> = None;
    let x = x.ok_or("bar");
    println!("{:?}", x);
    assert_eq!(x, Err("bar"));

    // ok_or_else() 将 Option<T> 转换为 Result<T,E>，将 Some(v) 映射为 Ok(v)，将 None 映射为 Err(err())。
    let x = Some("foo");
    let x = x.ok_or_else(|| 0);
    println!("{:?}", x);
    assert_eq!(x, Ok("foo"));

    let x: Option<&str> = None;
    let x = x.ok_or_else(|| "bar");
    println!("{:?}", x);
    assert_eq!(x, Err("bar"));

    // as_deref() 从 Option<T>（或 &Option<T>）转换为 Option<&T::Target>
    // 就地保留原选项，创建一个新选项，并引用原选项，另外通过 Deref 强制内容
    let x = Some("hey".to_owned());
    let x = x.as_deref();
    println!("{:?}", x);
    assert_eq!(x, Some("hey"));

    let x: Option<String> = None;
    let x = x.as_deref();
    println!("{:?}", x);
    assert_eq!(x, None);

    // as_deref_mut() 从 Option<T>（或 &mut Option<T>）转换为 Option<&mut T::Target>。
    // 就地保留原始 Option，创建一个新的 Option，其中包含对内部类型的 Deref::Target 类型的可变引用。
    let mut x = Some("hey".to_owned());
    let x = x.as_deref_mut().map(|x| {
        x.make_ascii_uppercase();
        x
    });
    println!("{:?}", x);
    assert_eq!(x, Some("HEY".to_owned().as_mut_str()));

    // iter() 返回可能包含的值的迭代器
    let x = Some(4);
    let x = x.iter().next();
    println!("{:?}", x);
    assert_eq!(x, Some(&4));

    // iter_mut() 返回可能包含的值的可变迭代器
    let mut x = Some(4);
    match x.iter_mut().next() {
        Some(v) => *v = 42,
        None => {}
    }
    println!("{:?}", x);
    assert_eq!(x, Some(42));

    let mut x: Option<u32> = None;
    let x = x.iter_mut().next();
    println!("{:?}", x);
    assert_eq!(x, None);

    // and() 如果选项为 None，则返回 None，否则返回参数的值 不是调用者的
    // 传递给 and 的参数是急于求值的；如果传递的是函数调用的结果，建议使用 and_then，它是懒于求值的
    let x = Some(2);
    let y: Option<&str> = None;
    let b = x.and(y);
    println!("{:?}", b);
    assert_eq!(b, None);

    let x: Option<u32> = None;
    let y = Some("foo");
    let b = x.and(y);
    println!("{:?}", b);
    assert_eq!(b, None);

    let x = Some(2);
    let y = Some("foo");
    let b = x.and(y);
    println!("{:?}", b);
    assert_eq!(b, Some("foo"));

    let x: Option<u32> = None;
    let y: Option<&u32> = None;
    let b = x.and(y);
    println!("{:?}", b);
    assert_eq!(b, None);

    // and_then() 如果选项为 None，则返回 None，否则使用封装值调用 f 并返回结果。
    // 有些语言称这种操作为 flatmap。
    let x = Some(2).and_then(sq_then_to_string);
    println!("{:?}", x);
    assert_eq!(x, Some(4.to_string()));

    // 函数调用的方法溢出 返回None
    let x = Some(1_000_000).and_then(sq_then_to_string);
    println!("{:?}", x);
    assert_eq!(x, None);

    let x = None.and_then(sq_then_to_string);
    println!("{:?}", x);
    assert_eq!(x, None);

    // 通常用于连锁可能返回 None 的错误操作。
    let arr_2d = [["A0", "A1"], ["B0", "B1"]];
    let item_0_1 = arr_2d.get(0).and_then(|row| row.get(1));
    println!("{:?}", item_0_1);
    assert_eq!(item_0_1, Some(&"A1"));

    let item_1_0 = arr_2d.get(1).and_then(|row| row.get(0));
    println!("{:?}", item_1_0);
    assert_eq!(item_1_0, Some(&"B0"));

    let item_2_0 = arr_2d.get(2).and_then(|row| row.get(0));
    println!("{:?}", item_2_0);
    assert_eq!(item_2_0, None);

    // filter() 如果选项为None，则返回None，否则使用包装后的值调用谓词并返回：
    assert_eq!(None.filter(is_even), None);
    let x = Some(3);
    let x = x.filter(is_even);
    println!("{:?}", x);
    assert_eq!(x, None);
    let x = Some(4);
    let x = x.filter(is_even);
    println!("{:?}", x);
    assert_eq!(x, Some(4));

    // or() 调用者和参数都不为None 返回调用者的值  都为None 返回None  调用者和参数有一个为None 返回另一个的值
    // 传递给 or 的参数是急于求值的；如果传递的是函数调用的结果，建议使用 or_else，它是懒于求值的。
    let x = Some(2);
    let y = None;
    assert_eq!(x.or(y), Some(2));

    let x = None;
    let y = Some(100);
    assert_eq!(x.or(y), Some(100));

    let x = Some(2);
    let y = Some(100);
    assert_eq!(x.or(y), Some(2));

    let x: Option<u32> = None;
    let y = None;
    assert_eq!(x.or(y), None);

    // or_else() 如果选项中包含值，则返回选项，否则调用 函数 并返回结果。
    let x = Some("barbarians").or_else(vikings);
    assert_eq!(x, Some("barbarians"));
    let x = None.or_else(vikings);
    assert_eq!(x, Some("vikings"));
    let x = None.or_else(nobody);
    assert_eq!(x, None);

    // xor() 如果调用者和参数有一个有值 返回该值  如果两个都有值或者都为None 返回None
    let x = Some(2);
    let y: Option<u32> = None;
    assert_eq!(x.xor(y), Some(2));

    let x: Option<u32> = None;
    let y = Some(2);
    assert_eq!(x.xor(y), Some(2));

    let x = Some(2);
    let y = Some(2);
    assert_eq!(x.xor(y), None);

    let x: Option<u32> = None;
    let y: Option<u32> = None;
    assert_eq!(x.xor(y), None);

    // insert() 将值插入选项，然后返回该值的可变引用。
    // 如果选项中已经包含一个值，旧值将被删除。
    let mut opt = None;
    let val = opt.insert(1);
    println!("{:?}", val);
    assert_eq!(*val, 1);
    assert_eq!(opt.unwrap(), 1);

    let val = opt.insert(2);
    *val = 3;
    assert_eq!(opt.unwrap(), 3);

    //  get_or_insert()  如果值为 None，则将其插入选项，然后返回所含值的可变引用。
    // 如果选项已包含某个值，则不会更新该值。
    let mut x = Some(10);
    {
        let y: &mut u32 = x.get_or_insert(5);
        assert_eq!(y, &10);
        *y = 7;
    }
    assert_eq!(x, Some(7));

    // get_or_insert_default() 如果缺省值为空，则将缺省值插入选项，然后返回所含值的可变引用。
    let mut x = None;
    {
        let y: &mut u32 = x.get_or_insert_default();
        assert_eq!(y, &0);
        *y = 7;
    }
    assert_eq!(x, Some(7));

    //  get_or_insert_with()  如果选项为空，则将根据 函数 计算得出的值插入该选项，然后返回所含值的可变引用。
    let mut x = None;
    {
        let y: &mut u32 = x.get_or_insert_with(|| 5);
        assert_eq!(y, &5);
        *y = 7;
    }
    assert_eq!(x, Some(7));

    // take() 删除选项中的值，留下一个 None 返回被删除元素
    let mut x = Some(2);
    let y = x.take();
    assert_eq!(x, None);
    assert_eq!(y, Some(2));

    let mut x: Option<u32> = None;
    let y = x.take();
    assert_eq!(x, None);
    assert_eq!(y, None);

    //  replace()  用参数中给定的值替换选项中的实际值，
    // 如果存在则返回旧值，在原处留下一个 Some 值，但不对其中任何一个值进行去初始化。
    let mut x = Some(2);
    let old = x.replace(5);
    assert_eq!(x, Some(5));
    assert_eq!(old, Some(2));

    let mut x = None;
    let old = x.replace(3);
    assert_eq!(x, Some(3));
    assert_eq!(old, None);

    // zip() Zips 自带另一个选项。
    // 如果自己是 Some(s)，对方是 Some(o)，此方法返回 Some((s,o))。否则，返回 None
    let x = Some(1);
    let y = Some("hi");
    let z = None::<u8>;
    assert_eq!(x.zip(y), Some((1, "hi")));
    assert_eq!(x.zip(z), None);

    // unzip() 解压缩包含两个元组的选项。
    // 如果 self 是 Some((a,b))，此方法会返回 (Some(a), Some(b))。否则，将返回 (None, None)
    let x = Some((1, "hi"));
    let y = None::<(u8, u32)>;

    assert_eq!(x.unzip(), (Some(1), Some("hi")));
    assert_eq!(y.unzip(), (None, None));

    //  copied()  通过复制选项的内容，将选项<&T>映射到选项<T>。
    let x = 12;
    let opt_x = Some(&x);
    assert_eq!(opt_x, Some(&12));
    let copied = opt_x.copied();
    assert_eq!(copied, Some(12));

    // cloned() 通过克隆选项的内容，将选项<&T>映射到选项<T>。
    let x = 12;
    let opt_x = Some(&x);
    assert_eq!(opt_x, Some(&12));
    let cloned = opt_x.cloned();
    assert_eq!(cloned, Some(12));

    //  copied() 通过复制选项的内容，将 Option<&mut T> 映射到 Option<T>。
    let mut x = 12;
    let opt_x = Some(&mut x);
    assert_eq!(opt_x, Some(&mut 12));
    let copied = opt_x.copied();
    assert_eq!(copied, Some(12));

    // cloned()通过克隆选项的内容，将选项<&mut T>映射为选项<T>。
    let mut x = 12;
    let opt_x = Some(&mut x);
    assert_eq!(opt_x, Some(&mut 12));
    let cloned = opt_x.cloned();
    assert_eq!(cloned, Some(12));

    //  transpose() 将Option<Result<T, E>> 转为 Result<Option<T>, E>
    // None 将被映射为 Ok(None)。Some(Ok(_)) 和 Some(Err(_)) 将被映射为 Ok(Some(_)) 和 Err(_)
    let x: Result<Option<i32>, SomeErr> = Ok(Some(5));
    let y: Option<Result<i32, SomeErr>> = Some(Ok(5));
    assert_eq!(x, y.transpose());

    //  flatten() 从 Option<Option<T>> 转换为 Option<T>。
    let x: Option<Option<u32>> = Some(Some(6));
    assert_eq!(Some(6), x.flatten());

    let x: Option<Option<u32>> = Some(None);
    assert_eq!(None, x.flatten());

    let x: Option<Option<u32>> = None;
    assert_eq!(None, x.flatten());

    // Flattening 每次只能移除一级嵌套：
    let x: Option<Option<Option<u32>>> = Some(Some(Some(6)));
    assert_eq!(Some(Some(6)), x.flatten());
    assert_eq!(Some(6), x.flatten().flatten());

}

#[derive(Debug, Eq, PartialEq)]
struct SomeErr;

fn nobody() -> Option<&'static str> {
    None
}

fn vikings() -> Option<&'static str> {
    Some("vikings")
}

fn is_even(n: &i32) -> bool {
    n % 2 == 0
}

fn sq_then_to_string(x: u32) -> Option<String> {
    x.checked_mul(x).map(|sq| sq.to_string())
}
