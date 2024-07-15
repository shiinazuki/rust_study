pub fn study_string() {
    // 新建字符串
    let mut _s = String::new();

    let data = "initial contents";
    let _s = data.to_string();
    let _s = "initial contents".to_string();

    let _s = String::from("initial contents");

    // 更新字符串
    let mut s = String::from("foo");
    s.push_str("bar");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    // push_str 方法采用字符串 slice，因为我们并不需要获取参数的所有权
    s1.push_str(s2);
    println!("s2 is {}", s2);

    // push() 只加单个字符
    let mut s = String::from("shiina");
    s.push('s');

    // 使用 + 运算符或 format! 宏拼接字符串
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let _s3 = s1 + &s2; // s1 被移动了 不能继续使用

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let _s = s1 + "-" + &s2 + "-" + &s3;

    // 对于更为复杂的字符串链接，可以使用 format! 宏

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let _s = format!("{}-{}-{}", s1, s2, s3);
    
    // 索引字符串 rust不支持

    // 字节、标量值和字形簇

    let hello = "hello";
    let _s = &hello[0..2];


    // 遍历字符串的方法
    for c in "shiina".chars() {
        print!("{}", c);
    }
    println!();
    for c in "zuki".bytes() {
        print!("{} ", c)
    }


}