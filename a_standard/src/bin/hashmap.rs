use std::{collections::HashMap, hash::{Hash, RandomState}};

fn main() {
    let mut book_reviews = HashMap::new();

    book_reviews.insert(
        "Adventures of Huckleberry Finn".to_string(),
        "My favorite book.".to_string(),
    );
    book_reviews.insert(
        "Grimms' Fairy Tales".to_string(),
        "Masterpiece.".to_string(),
    );
    book_reviews.insert(
        "Pride and Prejudice".to_string(),
        "Very enjoyable.".to_string(),
    );
    book_reviews.insert(
        "The Adventures of Sherlock Holmes".to_string(),
        "Eye lyked it alot.".to_string(),
    );
    if !book_reviews.contains_key("Les") {
        println!(
            "We've got {} reviews, but Les Misérables ain't one.",
            book_reviews.len()
        );
    }

    book_reviews.remove("The Adventures of Sherlock Holmes");

    let to_find = ["Pride and Prejudice", "Alice's Adventure in Wonderland"];
    for &book in &to_find {
        match book_reviews.get(book) {
            Some(review) => println!("{}: {}", book, review),
            None => println!("{} is unreviewed", book),
        }
    }
    for (book, review) in &book_reviews {
        println!("{}: \"{}\"", book, review);
    }

    // 带有已知项目列表的 HashMap 可以通过数组初始化
    let solar_distance = HashMap::from([
        ("Mercury", 0.4),
        ("Venus", 0.7),
        ("Earth", 1.0),
        ("Mars", 1.5),
    ]);
    println!("{:?}", solar_distance);

    // HashMap 实现了 Entry API，允许使用复杂的方法来获取、设置、更新和删除键及其值
    let mut player_stats = HashMap::new();
    fn random_stat_buff() -> u8 {
        42
    }

    // 仅在键不存在时才插入该键
    player_stats.entry("health").or_insert(100);

    //  使用一个函数插入一个键，该函数只有在键不存在时才提供新值
    player_stats
        .entry("defence")
        .or_insert_with(random_stat_buff);

    // 更新key，防止key可能未被设置
    let stat = player_stats.entry("attack").or_insert(100);
    *stat += random_stat_buff();

    // 在更新元素之前 如果key存在 则修改值  否则 返回默认的key
    player_stats
        .entry("mana")
        .and_modify(|mana| *mana += 200)
        .or_insert(100);

    println!("{}", player_stats.get("mana").unwrap());

    // 使用 HashMap 来存储 健康点数。
    let vikings = HashMap::from([
        (Viking::new("Einar", "Norway"), 25),
        (Viking::new("Olaf", "Denmark"), 24),
        (Viking::new("Harald", "Iceland"), 25),
    ]);

    for (viking, health) in &vikings {
        println!("{:?} has {} hp", viking, health)
    }

    // new() 创建一个空的 HashMap
    // 哈希映射创建之初的容量为 0，因此在首次插入之前不会分配
    let mut _map: HashMap<&str, i32> = HashMap::new();

    // with_capacity() 创建至少有指定容量的空 HashMap
    // 哈希映射将至少能够容纳容量元素，而无需重新分配。 该方法允许分配的元素数量超过容量。 如果容量为 0，散列映射将不会分配
    let mut _map: HashMap<&str, i32> = HashMap::with_capacity(10);

    // with_hasher() 创建一个空的 HashMap，使用给定的散列生成器对键进行散列
    // 创建的Map具有默认的初始容量
    let s = RandomState::new();
    let mut map = HashMap::with_hasher(s);
    map.insert(1, 2);

    // with_capacity_and_hasher() 创建一个至少有指定容量的空 HashMap，使用 hasher 对键进行散列
    let s = RandomState::new();
    let mut map = HashMap::with_capacity_and_hasher(10, s);
    map.insert(1, 2);

    // capacity() 返回map在不重新分配的情况下可容纳的元素数
    let map: HashMap<i32, i32> = HashMap::with_capacity(100);
    assert!(map.capacity() >= 100);

    // keys() 迭代器以任意顺序访问所有键。 迭代器元素类型为 &'a K
    let map = HashMap::from([("a", 1), ("b", 2), ("c", 3)]);
    for key in map.keys() {
        println!("{}", key);
    }
    // 在当前的实现中，遍历键需要 O(capacity) 时间，而不是 O(len) 时间，因为内部也会访问空桶

    // into_keys() 创建一个消耗迭代器，按任意顺序访问所有键。 调用此迭代器后，将无法使用map 迭代器元素类型为 K
    let map = HashMap::from([("a", 1), ("b", 2), ("c", 3)]);

    let mut vec: Vec<&str> = map.into_keys().collect();

    vec.sort_unstable();
    assert_eq!(vec, ["a", "b", "c"]);

    // values() 迭代器以任意顺序访问所有值。 迭代器元素类型为 &'a V
    let map = HashMap::from([("a", 1), ("b", 2), ("c", 3)]);

    for val in map.values() {
        println!("{}", val);
    }

    // values_mut() 迭代器以任意顺序可变地访问所有值。 迭代器的元素类型是 &'a mut V
    let mut map = HashMap::from([("a", 1), ("b", 2), ("c", 3)]);
    for val in map.values_mut() {
        *val = *val + 10;
    }
    for val in map.values() {
        println!("{}", val);
    }

    // into_values() 创建一个消耗迭代器，以任意顺序访问所有值。 调用此迭代器后，将无法使用map。 迭代器元素类型为 V
    let map = HashMap::from([("a", 1), ("b", 2), ("c", 3)]);

    let mut vec: Vec<i32> = map.into_values().collect();
    vec.sort_unstable();
    assert_eq!(vec, [1, 2, 3]);

    // iter() 迭代器以任意顺序访问所有键值对。 迭代器的元素类型是（&'a K, &'a V）
    let map = HashMap::from([("a", 1), ("b", 2), ("c", 3)]);

    for (key, val) in map.iter() {
        println!("key: {} val: {}", key, val);
    }

    // iter_mut() 迭代器以任意顺序访问所有键值对，并对值进行可变引用。 迭代器的元素类型是（&'a K, &'a mut V）
    let mut map = HashMap::from([("a", 1), ("b", 2), ("c", 3)]);

    for (_, val) in map.iter_mut() {
        *val *= 2;
    }
    
    for (key, val) in &map {
        println!("key: {} val: {}", key, val);
    }

    // len() 返图Map中元素的数量
    let mut a = HashMap::new();
    assert_eq!(a.len(), 0);
    a.insert(1, "a");
    assert_eq!(a.len(), 1);

    // is_empty() 如果地图不包含任何元素，则返回 true。
    let mut a = HashMap::new();
    assert!(a.is_empty());
    a.insert(1, "a");
    assert!(!a.is_empty());


    // drain() 清除映射，以迭代器形式返回所有键值对。 保留已分配的内存以供重复使用
    // 如果返回的迭代器在被完全消耗之前被丢弃，它就会丢弃剩余的键值对。 返回的迭代器会保留映射上的可变借用，以优化其执行
    let mut a = HashMap::new();
    a.insert(1, "a");
    a.insert(2, "b");


    for (k, v) in a.drain().take(1) {
        assert!(k == 1 || k == 2);
        assert!(v == "a" || v == "b");
    }
    assert!(a.is_empty());

    // retain() 只保留闭包指定的元素
    // 换句话说，删除 f(&k, &mut v) 返回 false 的所有元素对 (k, v)。 这些元素将以未排序（和未指定）的顺序被访问
    let mut map: HashMap<i32, i32> = (0..8).map(|x| (x, x * 10)).collect();
    map.retain(|&k, _| k % 2 == 0);
    assert_eq!(map.len(), 4);


    // clear() 清除映射，删除所有键值对。 保留已分配的内存以供重复使用
    let mut a = HashMap::new();
    a.insert(1, "a");
    a.clear();
    assert!(a.is_empty());


    // hasher() 返回Map BuildHasher 的引用
    let hasher = RandomState::new();
    let map: HashMap<i32, i32> = HashMap::with_hasher(hasher);
    let hasher: &RandomState = map.hasher();
    println!("{:?}", hasher);


    // reserve() 为至少更多元素插入 HashMap 预留空间。 为了避免频繁重新分配，集合可能会预留更多空间
    //  调用 reserve 后，容量将大于或等于 self.len() + additional。 如果容量已经足够，则不做任何操作
    let mut map: HashMap<&str, i32> = HashMap::new();
    map.reserve(10);


    // try_reserve() 尝试为至少更多元素插入 HashMap 预留空间。 集合可能会预留更多空间，以推测性地避免频繁重新分配
    // 调用 try_reserve 后，如果返回 Ok(())，容量将大于或等于 self.len() + additional。 如果容量已经足够，则不做任何操作。
    let mut map: HashMap<&str, isize> = HashMap::new();
    map.try_reserve(10).expect("why is the test harness OOMing on a handful of bytes?");

    // shrink_to_fit() 尽可能缩小Map的容量。 它会在保持内部规则的情况下尽可能缩小，并可能根据大小调整策略留出一些空间。
    let mut map: HashMap<i32, i32> = HashMap::with_capacity(100);
    map.insert(1, 2);
    map.insert(3, 4);
    assert!(map.capacity() >= 100);
    map.shrink_to_fit();
    assert!(map.capacity() >= 2);


    // shrink_to()  缩小Map容量的下限。 在保持内部规则的情况下，它的缩小幅度不会低于所提供的限制，并可能根据大小调整策略留出一些空间
    // 如果当前容量小于下限，则无法运行
    let mut map: HashMap<i32, i32> = HashMap::with_capacity(100);
    map.insert(1, 2);
    map.insert(3, 4);
    assert!(map.capacity() >= 100);
    map.shrink_to(10);
    assert!(map.capacity() >= 10);
    map.shrink_to(2);
    assert!(map.capacity() >= 2);


    // entry() 获取给定键在映射中的对应条目，以便进行就地操作
    let mut letters = HashMap::new();

    for ch in "a short treatise on fungi".chars() {
        letters.entry(ch).and_modify(|counter| *counter += 1).or_insert(1);
    }

    println!("{:?}", letters);

    assert_eq!(letters[&'s'], 2);
    assert_eq!(letters[&'t'], 3);
    assert_eq!(letters[&'u'], 1);
    assert_eq!(letters.get(&'y'), None);

    // get() 返回键对应的值的引用
    // 键可以是Map键类型的任何借用形式，但借用形式上的 Hash 和 Eq 必须与键类型相匹配
    let mut map = HashMap::new();
    map.insert(1, "a");
    assert_eq!(map.get(&1), Some(&"a"));
    assert_eq!(map.get(&2), None);

    // get_key_value() 返回所提供键 对应的键值对
    // 提供的键可以是映射键类型的任何借用形式，但借用形式上的 Hash 和 Eq 必须与键类型的 Hash 和 Eq 匹配
    let mut map = HashMap::new();
    map.insert(1, "a");
    assert_eq!(map.get_key_value(&1), Some((&1, &"a")));
    assert_eq!(map.get_key_value(&2), None);


    // contains_key() 如果Map包含指定键的值，则返回 true
    // 键可以是地图键类型的任何借用形式，但借用形式上的 Hash 和 Eq 必须与键类型相匹配
    let mut map = HashMap::new();
    map.insert(1, "A");
    assert_eq!(map.contains_key(&1), true);
    assert_eq!(map.contains_key(&2), false);

    // get_mut() 返回键对应的值的可变引用
    let mut map = HashMap::new();
    map.insert(1, "a");
    if let Some(x) = map.get_mut(&1) {
        *x = "b";
    }
    assert_eq!(map[&1], "b");

    // insert() 向Map中插入键值对 如果地图上没有该键，则返回 None 
    // 如果映射中确实有这个键，则会更新值，并返回旧值。 但键不会更新；这对可以 == 但不完全相同的类型很重要
    let mut map = HashMap::new();
    assert_eq!(map.insert(37, "a"), None);
    assert_eq!(map.is_empty(), false); 

    map.insert(37, "b");
    assert_eq!(map.insert(37, "c"), Some("b"));
    assert_eq!(map[&37], "c");


    // remove() 从映射表中移除键，如果键之前在映射表中，则返回键的值
    // 键可以是Map键类型的任何借用形式，但借用形式上的 Hash 和 Eq 必须与键类型相匹配
    let mut map = HashMap::new();
    map.insert(1, "a");
    assert_eq!(map.remove(&1), Some("a"));
    assert_eq!(map.remove(&1), None);

    // remove_entry() 从映射表中删除键，如果键之前在映射表中，则返回存储的键和值
    let mut map = HashMap::new();
    map.insert(1, "a");
    assert_eq!(map.remove_entry(&1), Some((1, "a")));
    assert_eq!(map.remove(&1), None);



}

// 将 HashMap 与自定义键类型结合使用的最简单方法是派生 Eq 和 Hash。 我们还必须派生 PartialEq

#[derive(Hash, Eq, PartialEq, Debug)]
struct Viking {
    name: String,
    country: String,
}

impl Viking {
    fn new(name: &str, country: &str) -> Viking {
        Viking {
            name: name.to_string(),
            country: country.to_string(),
        }
    }
}
