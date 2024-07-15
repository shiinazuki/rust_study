use std::collections::HashMap;

pub fn study_hashmap() {
    let mut scores = HashMap::new();

    // insert() 增加元素
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // get() 访问哈希 map 中的值 返回 Option<&V>，如果某个键在哈希 map 中没有对应的值，get 会返回 None
    // 调用 copied 方法来获取一个 Option<i32> 而不是 Option<&i32>
    // 调用 unwrap_or 在 scores 中没有该键所对应的项时将其设置为零
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("{}", score);

    for (key, value) in &scores {
        println!("key={}, value{}", key, value);
    }

    // 哈希 map 和所有权
    // 对于像 i32 这样的实现了 Copy trait 的类型，其值可以拷贝进哈希 map。
    // 对于像 String 这样拥有所有权的值，其值将被移动而哈希 map 会成为这些值的所有者
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // 这里 field_name 和 field_value 不再有效，
    // 尝试使用它们看看会出现什么编译错误！
    // 如果将值的引用插入哈希 map，这些值本身将不会被移动进哈希 map。但是这些引用指向的值必须至少在哈希 map 有效时也是有效的

    // 更新哈希 map
    // 覆盖一个值
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 50);
    println!("{:?}", scores);

    // entry() 只在键没有对应值时插入键值对
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    // Entry 的 or_insert 方法在键对应的值存在时就返回这个值的可变引用，如果不存在则将参数作为新值插入并返回新值的可变引用
    let val = scores.entry(String::from("Yellow")).or_insert(50);
    println!("val={}", val);
    let val = scores.entry(String::from("Blue")).or_insert(50);
    println!("val={}", val);

    println!("{:?}", scores);

    // 根据旧值更新一个值   找到一个键对应的值并根据旧的值更新它
    let text = "hello world wonderful world";

    let mut map = HashMap::new();
    let words = text.split_whitespace();
    for word in words {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);

    vect_exercise();
    str_exercise();
    department();
}

fn department() {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    println!("请输入命令（例如：Add Sally to Engineering 或者 List Engineering 或 List All）：");

    let str = "Add Sally to Engineering";
    let parse: Vec<&str> = str.split_whitespace().collect();
    match parse[0].to_lowercase().as_str() {
        "add" if parse.len() == 4 && parse[2].to_lowercase() == "to" => {
            let name = parse[1].to_string();
            let department = parse[3].to_string();

            let employees = map.entry(department).or_insert(Vec::new());
            employees.push(name);
            println!("已添加");
        }
        "list" if parse.len() == 2 => {
            let department = parse[1];
            if department.to_lowercase() == "all" {
                let mut all_employees: Vec<(&String, &String)> = Vec::new();
                for (dept, emps) in &map {
                    for emp in emps {
                        all_employees.push((dept, emp));
                    }
                }
                all_employees.sort_by(|a, b| a.1.cmp(b.1));
                println!("公司所有部门的所有员工:");
                for (dept, emp) in all_employees {
                    println!("部门={} 员工={}", dept, emp);
                }
            } else {
                match map.get(department) {
                    Some(employees) => {
                        let mut sorted_employees = employees.clone();
                        sorted_employees.sort();
                        println!("{} 部门的员工", department);
                        for employee in sorted_employees {
                            println!("员工={}", employee);
                        }
                    }
                    None => println!("部门 {} 不存在", department),
                }
            }
        }
        _ => println!("无效命令"),
    }
}

// 将字符串转换为 Pig Latin，也就是每一个单词的第一个辅音字母被移动到单词的结尾并增加 “ay”，所以 “first” 会变成 “irst-fay”
// 元音字母开头的单词则在结尾增加 “hay”（“apple” 会变成 “apple-hay”）。牢记 UTF-8 编码！
fn str_exercise() {
    let vowels = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
    let str = "first apple banana compute tender";
    let str = str
        .split_whitespace()
        .map(|word| {
            let first_char = word.chars().next().unwrap();
            if vowels.contains(&first_char) {
                format!("{}-hay", word)
            } else {
                let mut chars = word.chars();
                let first_consonant = chars.next().unwrap();
                let rest: String = chars.collect();
                format!("{}-{}ay", rest, first_consonant)
            }
        })
        .collect::<Vec<_>>()
        .join(" ");

    println!("{}", str);
}

// 给定一系列数字，使用 vector 并返回这个列表的中位数（排列数组后位于中间的值）和众数（mode，出现次数最多的值；这里哈希 map 会很有帮助)
fn vect_exercise() {
    let mut vec = vec![
        39, 1, 45, 69, 67, 12, 45, 56, 94, 100, 1, 1, 1, 3, 3, 3, 5, 7, 8, 9, 43, 34, 23, 54, 65,
        67, 423, 56, 67,
    ];

    // median
    bubble_sort(&mut vec);
    let mid = vec.len() / 2;
    let median = if vec.len() % 2 == 0 {
        (vec[mid - 1] + vec[mid]) as f64 / 2.0
    } else {
        vec[mid] as f64
    };
    println!("median={}", median);

    // mode
    let mut map = HashMap::new();
    for i in vec {
        *map.entry(i).or_insert(0) += 1;
    }

    let mode = map
        .iter()
        .max_by_key(|&(_, v)| v)
        .map(|(&k, _)| k)
        .expect("Cannot compute the mode of zero numbers");

    println!("mode={}", mode);
}

fn bubble_sort(vec: &mut Vec<i32>) {
    for i in 0..vec.len() {
        for j in 0..vec.len() - i - 1 {
            if vec[j] > vec[j + 1] {
                let temp = vec[j];
                vec[j] = vec[j + 1];
                vec[j + 1] = temp;
            }
        }
    }
}
