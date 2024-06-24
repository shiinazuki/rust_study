fn main() {
    // slice 允许你引用集合中一段连续的元素序列，而不用引用整个集合。
    // slice 是一种引用，所以它没有所有权。

    let s = String::from("hello world");

    let world = first_word(&s); // word 的值为 5

    println!("{}", world);

    // 当拥有某值的不可变引用时，就不能再获取一个可变引用。因为 clear 需要清空 String，它尝试获取一个可变引用
    // s.clear(); // 这清空了字符串，使其等于 ""

    // 后面又使用了 world  所以根据借用规则  这是不允许的
    // Rust 不允许 clear 中的可变引用和 word 中的不可变引用同时存在，因此编译失败
    println!("{}", world);

    // word 在此处的值仍然是 5，
    // 但是没有更多的字符串让我们可以有效地应用数值 5。word 的值现在完全无效！

    // 字符串 slice
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    println!("{}", hello);
    println!("{}", world);

    let my_string = String::from("hello world");

    // `first_word` 适用于 `String`（的 slice），部分或全部
    let word = first_word(&my_string[0..6]);
    println!("{}", word);
    let word = first_word(&my_string[..]);
    println!("{}", word);

    // `first_word` 也适用于 `String` 的引用，
    // 这等价于整个 `String` 的 slice
    let word = first_word(&my_string);
    println!("{}", word);

    let my_string_literal = "hello world";

    // `first_word` 适用于字符串字面值，部分或全部
    let word = first_word(&my_string_literal[0..6]);
    println!("{}", word);

    let word = first_word(&my_string_literal[..]);
    println!("{}", word);

    // 因为字符串字面值已经 **是** 字符串 slice 了，
    // 这也是适用的，无需 slice 语法！
    let word = first_word(my_string_literal);
    println!("{}", word);


    // 其他类型的 slice
    let a = [1, 2, 3, 4, 5 ];
    let array_slice = &a[1..4];
    println!("{:?}", array_slice);


}

fn first_word(s: &str) -> &str {
    // fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
