use std::slice;
use std::str;

fn main() {
    // 字符串字面量是字符串切片
    let _hello_world = "Hello, World";
    let _hello_world: &'static str = "Hello, World";

    // 一个 &str 由两个部分组成：一个指向某些字节的指针和一个长度。
    // 可以使用 as_ptr 和 len 方法查看它们：
    let story = "Once upon a time...";
    let ptr = story.as_ptr();
    let len = story.len();
    assert_eq!(19, len);
    let s = unsafe {
        // 我们可以用 ptr 和 len 重新构建一个字符串 这是不安全的 因为我们有责任确保这两个部分是有效的：
        let slice = slice::from_raw_parts(ptr, len);

        str::from_utf8(slice)
    };

    assert_eq!(s, Ok(story));

    // len() 返回 self 的长度，长度单位是字节，而不是字符或音节  它可能不是人类认为的字符串长度。
    let len = "foo".len();
    assert_eq!(3, len);
    assert_eq!("ƒoo".len(), 4);
    assert_eq!("ƒoo".chars().count(), 3);
    // is_empty() 如果 self 的长度为零字节，则返回 true
    let s = "";
    assert!(s.is_empty());
    let s = "not_empty";
    assert!(!s.is_empty());

    // is_char_boundary() 检查 index-th 字节是 UTF-8 代码点序列中的第一个字节还是字符串的结尾
    // 字符串的开始和结尾（当 index == self.len() 时）被视为边界
    // 如果 index 大于 self.len()，则返回 false
    /*
        如果索引是字符串的起始位置（0），则返回 true。
        如果索引是字符串的末尾位置（self.len()），则返回 true。
        如果索引位置是某个 UTF-8 字符的第一个字节，则返回 true。
        如果索引位置在字符串范围之外，则返回 false。
        如果索引位置是某个 UTF-8 字符的中间字节，则返回 false。
    */
    let s = "Löwe 老虎 Léopard";
    assert!(s.is_char_boundary(0));
    // start of `老`
    assert!(s.is_char_boundary(6));
    assert!(s.is_char_boundary(s.len()));

    // second byte of `ö`
    assert!(!s.is_char_boundary(2));

    // third byte of `老`
    assert!(!s.is_char_boundary(8));

    // as_bytes() 将字符串片段转换为字节片段。 要将字节片段转换回字符串片段，请使用 from_utf8 函数。
    let bytes = "bors".as_bytes();
    assert_eq!(b"bors", bytes);
    let str = String::from_utf8(bytes.to_vec());
    println!("{:?}", str);

    // as_bytes_mut() 将可变字符串片段转换为可变字节片段
    let mut s = String::from("Hello");
    let bytes = unsafe { s.as_bytes_mut() };
    assert_eq!(b"Hello", bytes);
    bytes[0] = b'W';
    let str = String::from_utf8(bytes.to_vec());
    println!("{:?}", str);

    // as_ptr() 将字符串片段转换为原始指针
    // 由于字符串片是一个字节片，原始指针指向一个 u8。 调用者必须确保返回的指针不会被写入。
    // 如果需要更改字符串片的内容，请使用 as_mut_ptr
    let s = "Hello";
    let _ptr = s.as_ptr();

    // as_mut_ptr() 将可变字符串片段转换为原始指针

    // get() 返回 str 的子片段
    // 这是索引字符串的非恐慌替代方法。 如果同等的索引操作会引起恐慌，则返回 None
    let v = String::from("hello, world");
    assert_eq!(Some("hello"), v.get(0..5));
    // 索引不在范围上 返回None
    assert!(!v.get(1..).is_none());
    assert!(v.get(..20).is_none());

    // get_mut() 返回 str 的一个可变子片段
    // 这是索引字符串的非恐慌替代方法。 如果同等的索引操作会引起恐慌，则返回 None
    let mut v = String::from("hello");
    assert!(v.get_mut(0..5).is_some());
    assert!(v.get_mut(..43).is_none());
    assert_eq!(Some("he"), v.get_mut(0..2).map(|v| &*v));

    assert_eq!("hello", v);
    {
        let s = v.get_mut(0..2);
        let s = s.map(|s| {
            s.make_ascii_uppercase();
            &*s
        });
        assert_eq!(Some("HE"), s);
    }
    assert_eq!("HEllo", v);

    // get_unchecked() 返回 str 的未选中子片段，这是索引 str 的未选中替代方法
    /*
        起始索引不得超过终止索引；索引必须在原始片段的范围内；索引必须位于 UTF-8 序列边界上。
        否则，返回的字符串片段可能会引用无效内存或违反 str 类型所传达的不变式。
    */
    let v = "hello, world";
    unsafe {
        assert_eq!("hello", v.get_unchecked(0..5));
        assert_eq!("world", v.get_unchecked(7..));
    }

    // get_unchecked_mut() 返回 str 的一个可变、未选中的子片段。 这是索引 str 的未选中替代方法
    let mut v = String::from("hello, world");
    unsafe {
        let v1 = v.get_unchecked_mut(0..6);
        println!("{}", v1);
        // 不能直接将一个字符串字面量分配给一个可变借用的字符串切片
        // 要修改字符串切片的内容，需要通过字节数组的形式进行修改
        let v1 = v1.as_bytes_mut();
        v1.copy_from_slice(b"world!");
        v1[5] = b'.';
    }
    println!("{}", v);

    // split_at() 按索引将一个字符串切片分成两个
    let s = "Per Martin-Lof";
    let (first, last) = s.split_at(3);
    assert_eq!(first, "Per");
    assert_eq!(last, " Martin-Lof");

    // split_at_mut() 在一个索引处将一个可变字符串片段分成两个
    let mut s = "Per Martin-Lof".to_string();
    {
        let (first, last) = s.split_at_mut(3);
        first.make_ascii_uppercase();
        assert_eq!(first, "PER");
        assert_eq!(last, " Martin-Lof");
    }
    assert_eq!("PER Martin-Lof", s);

    // chars() 返回一个字符串片段的迭代器
    let word = "goodbye";
    let count = word.chars().count();
    assert_eq!(count, 7);

    let mut chars = word.chars();

    assert_eq!(Some('g'), chars.next());
    assert_eq!(Some('o'), chars.next());
    assert_eq!(Some('o'), chars.next());
    assert_eq!(Some('d'), chars.next());
    assert_eq!(Some('b'), chars.next());
    assert_eq!(Some('y'), chars.next());
    assert_eq!(Some('e'), chars.next());
    assert_eq!(None, chars.next());
    // 记住，字符可能与你对字符的直觉不符
    let y = "y̆";
    let mut chars = y.chars();

    assert_eq!(Some('y'), chars.next()); // not 'y̆'
    assert_eq!(Some('\u{0306}'), chars.next());

    assert_eq!(None, chars.next());

    // char_indices() 返回字符串片段中字符及其位置的迭代器
    // 迭代器产生元组。 位置在前，字符在后
    let word = "goodbye";

    let count = word.chars().count();
    assert_eq!(7, count);
    let mut char_indices = word.char_indices();

    assert_eq!(Some((0, 'g')), char_indices.next());
    assert_eq!(Some((1, 'o')), char_indices.next());
    assert_eq!(Some((2, 'o')), char_indices.next());
    assert_eq!(Some((3, 'd')), char_indices.next());
    assert_eq!(Some((4, 'b')), char_indices.next());
    assert_eq!(Some((5, 'y')), char_indices.next());
    assert_eq!(Some((6, 'e')), char_indices.next());

    assert_eq!(None, char_indices.next());

    // bytes() 字符串片段字节的迭代器
    // 由于字符串片段由字节序列组成，我们可以按字节遍历字符串片段。 本方法返回这样一个迭代器
    let mut bytes = "bors".bytes();
    assert_eq!(Some(b'b'), bytes.next());
    assert_eq!(Some(b'o'), bytes.next());
    assert_eq!(Some(b'r'), bytes.next());
    assert_eq!(Some(b's'), bytes.next());

    assert_eq!(None, bytes.next());

    // split_whitespace() 按空白分割字符串片段
    // 迭代器返回的字符串片段是原始字符串片段的子片段，之间用任意数量的空白分隔
    // 空白 "是根据 Unicode 派生核心属性 White_Space 的条款定义的。 如果只想分割 ASCII 空白，请使用 split_ascii_whitespace
    let mut iter = "A few words".split_whitespace();

    assert_eq!(Some("A"), iter.next());
    assert_eq!(Some("few"), iter.next());
    assert_eq!(Some("words"), iter.next());
    assert_eq!(None, iter.next());
    // 各种空白都考虑在内
    let mut iter = " Mary   had\ta\u{2009}little  \n\t lamb".split_whitespace();
    assert_eq!(Some("Mary"), iter.next());
    assert_eq!(Some("had"), iter.next());
    assert_eq!(Some("a"), iter.next());
    assert_eq!(Some("little"), iter.next());
    assert_eq!(Some("lamb"), iter.next());

    assert_eq!(None, iter.next());

    // 如果字符串为空或全是空白，迭代器不会产生字符串片段
    assert_eq!("".split_whitespace().next(), None);
    assert_eq!("   ".split_whitespace().next(), None);

    // split_ascii_whitespace() 按 ASCII 空格分割字符串片段
    // 迭代器返回的字符串片段是原始字符串片段的子片段，之间用任意数量的 ASCII 空格分隔
    let mut iter = "A few words".split_ascii_whitespace();

    assert_eq!(Some("A"), iter.next());
    assert_eq!(Some("few"), iter.next());
    assert_eq!(Some("words"), iter.next());

    assert_eq!(None, iter.next());

    // 所有类型的 ASCII 空格都在考虑之列
    let mut iter = " Mary   had\ta little  \n\t lamb".split_ascii_whitespace();
    assert_eq!(Some("Mary"), iter.next());
    assert_eq!(Some("had"), iter.next());
    assert_eq!(Some("a"), iter.next());
    assert_eq!(Some("little"), iter.next());
    assert_eq!(Some("lamb"), iter.next());
    assert_eq!(None, iter.next());

    // 如果字符串为空或全是 ASCII 白字符，迭代器不会产生字符串片段
    assert_eq!("".split_ascii_whitespace().next(), None);
    assert_eq!("   ".split_ascii_whitespace().next(), None);

    // lines() 字符串各行的迭代器，如同字符串切片  在换行 (\n)或回车后换行 (\r\n)的行尾处分行
    // 迭代器返回的行中不包括行结束符
    // 请注意，任何回车 (\r)，如果没有紧跟换行 (\n)，都不会分行。 因此，这些回车符也包含在生成的行中
    // 最后一行结尾为可选项。 以最后一行结尾结束的字符串将返回与没有最后一行结尾的相同字符串相同的行
    let text = "foo\r\nbar\n\nbaz\r";
    let mut lines = text.lines();

    assert_eq!(Some("foo"), lines.next());
    assert_eq!(Some("bar"), lines.next());
    assert_eq!(Some(""), lines.next());
    assert_eq!(Some("baz\r"), lines.next());
    assert_eq!(None, lines.next());

    // 最后一行不需要任何结尾
    let text = "foo\nbar\n\r\nbaz";
    let mut lines = text.lines();

    assert_eq!(Some("foo"), lines.next());
    assert_eq!(Some("bar"), lines.next());
    assert_eq!(Some(""), lines.next());
    assert_eq!(Some("baz"), lines.next());

    assert_eq!(None, lines.next());

    // contains() 如果给定模式匹配此字符串片的子片，则返回 true
    // 如果不匹配，则返回 false 模式可以是字符串、字符、字符片段，也可以是判断字符是否匹配的函数或闭包
    let bananas = "bananas";
    assert!(bananas.contains("nana"));
    assert!(!bananas.contains("apples"));

    // starts_with() 如果给定模式匹配此字符串片的前缀，则返回 true；如果不匹配，则返回 false
    // 模式也可以是一个字符、一个字符片段，或者是一个判断字符是否匹配的函数或闭包。
    // 这些将只针对该字符串片的第一个字符进行检查。 请看下面关于字符片行为的第二个示例。
    let bananas = "bananas";
    assert!(bananas.starts_with("bana"));
    assert!(!bananas.starts_with("nana"));

    let bananas = "bananas";

    //  请注意，这两个断言都成功了
    /*
        starts_with(&['b', 'a', 'n', 'a']) 和 starts_with(&['a', 'b', 'c', 'd'])
        会检查字符串 bananas 是否以 ['b', 'a', 'n', 'a'] 或 ['a', 'b', 'c', 'd'] 中的任何一个字符开始
    */
    assert!(bananas.starts_with(&['b', 'a', 'n', 'a']));
    assert!(bananas.starts_with(&['a', 'b', 'c', 'd']));

    // ends_with() 如果给定模式匹配此字符串片段的后缀，则返回 true  否则返回 false
    // 模式可以是字符串、字符、字符片段，也可以是判断字符是否匹配的函数或闭包
    let bananas = "bananas";
    assert!(bananas.ends_with("anas"));
    assert!(!bananas.ends_with("nana"));

    // find() 返回该字符片中与模式匹配的第一个字符的字节索引 如果模式不匹配，则返回 None
    // 模式可以是字符串、字符、字符片段，也可以是判断字符是否匹配的函数或闭包
    let s = "hello, world";
    assert_eq!(s.find('l'), Some(2));
    assert_eq!(s.find("orld"), Some(8));
    assert_eq!(s.find("patt"), None);

    // 使用无点样式和闭包的更复杂模式
    let s = "Hello world aaa";
    assert_eq!(s.find(char::is_whitespace), Some(5));
    assert_eq!(s.find(char::is_lowercase), Some(1));
    assert_eq!(
        s.find(|c: char| c.is_whitespace() || c.is_lowercase()),
        Some(1)
    );
    assert_eq!(s.find(|c: char| (c < 'o') && (c > 'a')), Some(1));

    // 找不到模式
    let s = "hello, world";
    let x: &[_] = &['1', '2'];

    assert_eq!(s.find(x), None);

    // rfind()  返回该字符串片段中最后一个匹配模式的第一个字符的字节索引 如果模式不匹配，则返回 None
    // 模式可以是字符串、字符、字符片段，也可以是判断字符是否匹配的函数或闭包
    let s = "hello, world";
    assert_eq!(s.rfind("o"), Some(8));
    assert_eq!(s.rfind("ld"), Some(10));

    // s.split() 该字符串片段子串的迭代器，由模式匹配的字符分隔
    let v: Vec<&str> = "Mary had a little lamb".split(' ').collect();
    assert_eq!(v, ["Mary", "had", "a", "little", "lamb"]);

    let v: Vec<&str> = "".split('X').collect();
    assert_eq!(v, [""]);

    let v: Vec<&str> = "lionXXtigerXleopard".split('X').collect();
    assert_eq!(v, ["lion", "", "tiger", "leopard"]);

    let v: Vec<&str> = "lion::tiger::leopard".split("::").collect();
    assert_eq!(v, ["lion", "tiger", "leopard"]);

    let v: Vec<&str> = "abc1def2ghi".split(char::is_numeric).collect();
    assert_eq!(v, ["abc", "def", "ghi"]);

    let v: Vec<&str> = "lionXtigerXleopard".split(char::is_uppercase).collect();
    assert_eq!(v, ["lion", "tiger", "leopard"]);

    // 如果模式是一片字符，则在每出现一个字符时进行分割
    let v: Vec<&str> = "2020-11-03 23:59"
        .split(&['-', ' ', ':', '@'][..])
        .collect();
    assert_eq!(v, ["2020", "11", "03", "23", "59"]);

    let v: Vec<&str> = "abc1defXghi".split(|c| c == '1' || c == 'X').collect();
    assert_eq!(v, ["abc", "def", "ghi"]);

    // 如果字符串包含多个连续分隔符，输出结果将是空字符串
    let x = "||||a||b|c".to_string();
    let d: Vec<_> = x.split('|').collect();

    assert_eq!(d, &["", "", "", "", "a", "", "b", "c"]);

    // 连续分隔符由空字符串分隔
    let x = "(///)".to_string();
    let d: Vec<_> = x.split('/').collect();

    assert_eq!(d, &["(", "", "", ")"]);

    // 当空字符串用作分隔符时，它会分隔字符串中的每个字符以及字符串的开头和结尾
    let f: Vec<_> = "rust".split("").collect();
    assert_eq!(f, &["", "r", "u", "s", "t", ""]);

    let x = "    a  b c".to_string();
    let d: Vec<_> = x.split(' ').collect();

    assert_eq!(d, &["", "", "", "", "a", "", "b", "c"]);

    // split_inclusive() 该字符串片段子串的迭代器，由模式匹配的字符分隔。
    // 与 split 产生的迭代器不同的是，split_inclusive 将匹配的部分作为子串的终止符

    let v: Vec<&str> = "Mary had a little lamb\nlittle lamb\nlittle lamb."
        .split_inclusive('\n')
        .collect();
    assert_eq!(
        v,
        ["Mary had a little lamb\n", "little lamb\n", "little lamb."]
    );

    // 如果匹配到字符串的最后一个元素，该元素将被视为前面子串的终止符。 该子串将是迭代器返回的最后一个项目
    let v: Vec<&str> = "Mary had a little lamb\nlittle lamb\nlittle lamb.\n"
        .split_inclusive('\n')
        .collect();
    assert_eq!(
        v,
        [
            "Mary had a little lamb\n",
            "little lamb\n",
            "little lamb.\n"
        ]
    );

    // capacity() 返回字符串的容量（以字节为单位）
    let s = String::with_capacity(10);
    assert!(s.capacity() >= 10);

    // matches()  在给定的字符串片段中，模式的不相邻匹配项的迭代器
    let v: Vec<&str> = "abcXXXabcYYYabc".matches("abc").collect();
    assert_eq!(v, ["abc", "abc", "abc"]);

    let v: Vec<&str> = "1abc2abc3".matches(char::is_numeric).collect();
    assert_eq!(v, ["1", "2", "3"]);

    // rmatches() 该字符串片段中模式的不相邻匹配项的迭代器，以相反顺序生成
    let v: Vec<&str> = "abcXXXabcYYYabc".rmatches("abc").collect();
    assert_eq!(v, ["abc", "abc", "abc"]);

    let v: Vec<&str> = "1abc2abc3".rmatches(char::is_numeric).collect();
    assert_eq!(v, ["3", "2", "1"]);

    // parse() 将此字符串片段解析为另一种类型
    let four: u32 = "4".parse().unwrap();
    assert_eq!(4, four);
    // 使用 涡轮鱼 语法
    let four = "4".parse::<u32>();
    assert_eq!(Ok(4), four);

    // let nope = "j".parse::<32>();
    // assert!(nope.is_err());

    // replace()  用另一个字符串替换模式的所有匹配字符串
    let s = "this is old";
    assert_eq!("this is new", s.replace("old", "new"));
    assert_eq!("than an old", s.replace("is", "an"));

    // 当模式不匹配时，它会将该字符串片段返回为字符串
    let s = "this is old";
    assert_eq!(s, s.replace("cookie monster", "little lamb"));

    // replacen()  用另一个字符串替换模式的前 N 个匹配字符串
    let s = "foo foo 123 foo";
    assert_eq!("new new 123 new", s.replacen("foo", "new", 3));
    assert_eq!("faa fao 123 foo", s.replacen('o', "a", 3));
    assert_eq!("foo foo new23 foo", s.replacen(char::is_numeric, "new", 1));

    // 当模式不匹配时，它会将该字符串片段返回为字符串
    let s = "this is old";
    assert_eq!(s, s.replacen("cookie monster", "little lamb", 10));

    // into_string() 将Box<str> 转换为字符串，无需复制或分配
    let string = String::from("birthday gift");
    let boxed_str = string.clone().into_boxed_str();

    assert_eq!(boxed_str.into_string(), string);


    // repeat() 将一个字符串重复 n 次，创建一个新字符串
    assert_eq!("abc".repeat(4), String::from("abcabcabcabc"));
}
