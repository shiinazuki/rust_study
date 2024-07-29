use std::{
    cmp::Ordering,
    collections::VecDeque,
    fs,
    io::{stdout, Write},
    iter::zip,
    ops::ControlFlow,
    path::Path,
    sync::mpsc,
};

fn main() {
    // next() 推进迭代器并返回下一个值
    let a = [1, 2, 3];

    let mut iter = a.iter();
    assert_eq!(Some(&1), iter.next());
    assert_eq!(Some(&2), iter.next());
    assert_eq!(Some(&3), iter.next());

    assert_eq!(None, iter.next());

    // 更多调用可能返回`None`，也可能不返回`None`。 在这里，它们总是会返回
    assert_eq!(None, iter.next());
    assert_eq!(None, iter.next());

    // size_hint() 返回迭代器剩余长度的边界
    // 具体来说，size_hint() 返回一个元组，其中第一个元素是下限，第二个元素是上限
    // 返回元组的后半部分是一个 Option<usize>。 这里的 None 意味着要么没有已知的上限，要么上限大于 usize
    let a = [1, 2, 3];
    let mut iter = a.iter();
    assert_eq!((3, Some(3)), iter.size_hint());
    let _ = iter.next();
    assert_eq!((2, Some(2)), iter.size_hint());

    // 我们可能会从零到十遍历。 如果不执行 filter() 命令，就不可能知道是五次
    let iter = (0..10).filter(|x| x % 2 == 0);
    assert_eq!((0, Some(10)), iter.size_hint());

    // 用 chain() 增加五个数字
    let iter = (0..10).filter(|x| x % 2 == 0).chain(15..20);

    // 现在两个边界都增加了 5
    assert_eq!((5, Some(15)), iter.size_hint());

    let iter = 0..;
    assert_eq!((usize::MAX, None), iter.size_hint());

    // count() 消耗迭代器，计算迭代次数并返回
    // 该方法将重复调用 next，直到遇到 None，并返回看到 Some 的次数
    // 请注意，即使迭代器中没有任何元素，也必须至少调用一次 next
    let a = [1, 2, 3];
    assert_eq!(a.iter().count(), 3);

    let a = [1, 2, 3, 4, 5];
    assert_eq!(a.iter().count(), 5);

    // last()  消耗迭代器，返回最后一个元素
    let a = [1, 2, 3];
    assert_eq!(a.iter().last(), Some(&3));

    let a = [1, 2, 3, 4, 5];
    assert_eq!(a.iter().last(), Some(&5));

    // nth()  返回迭代器的第 n 个元素  计数从零开始  n 大于迭代器的长度 返回None
    // 请注意，所有前面的元素以及返回的元素都将从迭代器中消耗掉。
    // 这意味着前面的元素将被丢弃，而且在同一个迭代器上多次调用 nth(0) 将返回不同的元素
    let a = [1, 2, 3];
    assert_eq!(a.iter().nth(1), Some(&2));

    let a = [1, 2, 3];
    let mut iter = a.iter();

    assert_eq!(iter.nth(1), Some(&2));
    println!("{:?}", iter);
    assert_eq!(iter.nth(1), None);

    let a = [1, 2, 3];
    assert_eq!(a.iter().nth(10), None);

    // step_by() 创建一个迭代器，从相同的点开始 每次按给定的步长迭代
    // 无论给定的步长是多少，迭代器的第一个元素总是会返回
    // 如果给定的步长为 0，该方法就会崩溃
    let a = [0, 1, 2, 3, 4, 5];
    let mut iter = a.iter().step_by(2);

    assert_eq!(iter.next(), Some(&0));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&4));
    assert_eq!(iter.next(), None);

    // chain() 接收两个迭代器，并依次在两个迭代器上创建一个新的迭代器
    // chain() 将返回一个新的迭代器，首先遍历第一个迭代器中的值，然后遍历第二个迭代器中的值
    // 一次通常用于将单个值调整为其他类型的迭代链
    let a1 = [1, 2, 3];
    let a2 = [4, 5, 6];

    let mut iter = a1.iter().chain(a2.iter());

    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), Some(&4));
    assert_eq!(iter.next(), Some(&5));
    assert_eq!(iter.next(), Some(&6));
    assert_eq!(iter.next(), None);

    // 由于 chain() 的参数使用 IntoIterator，因此我们可以传递任何可以转换为迭代器的内容，而不仅仅是迭代器本身
    let s1 = &[1, 2, 3];
    let s2 = &[4, 5, 6];

    let mut iter = s1.iter().chain(s2);
    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), Some(&4));
    assert_eq!(iter.next(), Some(&5));
    assert_eq!(iter.next(), Some(&6));
    assert_eq!(iter.next(), None);

    // zip() 将两个迭代器 压缩 为一个成对迭代器  返回一个新迭代器
    // 如果任一迭代器返回 None，则下一个来自压缩迭代器的迭代器也将返回 None
    // 如果压缩迭代器没有更多元素可返回，那么每次尝试向前推进时，都会首先尝试向前推进第一个迭代器最多一次，
    // 如果仍有一个项目，则尝试向前推进第二个迭代器最多一次

    let a1 = [1, 2, 3];
    let a2 = [4, 5, 6];

    let mut iter = a1.iter().zip(a2.iter());

    assert_eq!(iter.next(), Some((&1, &4)));
    assert_eq!(iter.next(), Some((&2, &5)));
    assert_eq!(iter.next(), Some((&3, &6)));
    assert_eq!(iter.next(), None);

    // 可以给zip传切片
    let s1 = &[1, 2, 3];
    let s2 = &[4, 5, 6];
    let mut iter = s1.iter().zip(s2);

    assert_eq!(iter.next(), Some((&1, &4)));
    assert_eq!(iter.next(), Some((&2, &5)));
    assert_eq!(iter.next(), Some((&3, &6)));
    assert_eq!(iter.next(), None);

    // zip() 常用于将无限迭代器压缩为有限迭代器。
    // 这是因为有限迭代器最终会返回 None，从而结束拉链。 使用 (0...) 进行拉链操作看起来很像 enumerate
    let enumerate: Vec<_> = "foo".chars().enumerate().collect();

    let zipper: Vec<_> = (0..).zip("foo".chars()).collect();

    assert_eq!((0, 'f'), enumerate[0]);
    assert_eq!((0, 'f'), zipper[0]);

    assert_eq!((1, 'o'), enumerate[1]);
    assert_eq!((1, 'o'), zipper[1]);

    assert_eq!((2, 'o'), enumerate[2]);
    assert_eq!((2, 'o'), zipper[2]);

    // 如果两个迭代器的语法大致相同，使用 zip 文件可能更易读
    let a = [1, 2, 3];
    let b = [2, 3, 4];

    let mut zipped = zip(
        a.into_iter().map(|x| x * 2).skip(1),
        b.into_iter().map(|x| x * 2).skip(1),
    );

    assert_eq!(zipped.next(), Some((4, 6)));
    assert_eq!(zipped.next(), Some((6, 8)));
    assert_eq!(zipped.next(), None);

    let mut zipped = a
        .into_iter()
        .map(|x| x * 2)
        .skip(1)
        .zip(b.into_iter().map(|x| x * 2).skip(1));

    assert_eq!(zipped.next(), Some((4, 6)));
    assert_eq!(zipped.next(), Some((6, 8)));
    assert_eq!(zipped.next(), None);

    // map() 获取一个闭包，并创建一个迭代器，在每个元素上调用该闭包
    let a = [1, 2, 3];
    let mut iter = a.iter().map(|x| x * 2);

    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), Some(4));
    assert_eq!(iter.next(), Some(6));
    assert_eq!(iter.next(), None);

    // 如果您正在执行某种副作用，请首选 map()
    // 什么也没做
    let _ = (0..5).map(|x| println!("{}", x));

    // 打印 0 ~ 4
    for x in 0..5 {
        println!("{}", x);
    }

    // for_each() 在迭代器的每个元素上调用闭包
    // 这等同于在迭代器上使用 for 循环，尽管在闭包中无法使用 break 和 continue。
    // 一般来说，使用 for 循环更容易理解，但在处理较长迭代器链末尾的项目时，for_each 可能更容易理解。
    // 在某些情况下，for_each 也可能比循环更快，因为它会在适配器上使用内部迭代，如 Chain

    let (tx, rx) = mpsc::channel();
    (0..5)
        .map(|x| x * 2 + 1)
        .for_each(move |x| tx.send(x).unwrap());

    let v: Vec<_> = rx.iter().collect();
    assert_eq!(v, vec![1, 3, 5, 7, 9]);

    // 对于这样一个小例子，for 循环可能更简洁，但为了保持功能风格，for_each 可能更适合使用较长的迭代器
    (0..5)
        .flat_map(|x| x * 100..x * 110)
        .enumerate()
        .filter(|&(i, x)| (i + x) % 3 == 0)
        .for_each(|(i, x)| println!("{i}:{x}"));

    // filter() 创建一个迭代器，该迭代器使用闭包来确定是否应该让渡某个元素
    // 给定一个元素，闭包必须返回 true 或 false。 返回的迭代器将只产生闭包返回 true 的元素
    // 0 是非负数 不是正数
    let a = [0_i32, 1, 2];
    let mut iter = a.iter().filter(|x| x.is_positive());
    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), None);

    let a = [0, 1, 2];
    let mut iter = a.iter().filter(|x| **x > 1);
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), None);

    // 通常的做法是在参数上使用解构来去掉一个
    let a = [0, 1, 2];
    let mut iter = a.iter().filter(|&x| *x > 1);
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), None);

    // or
    let a = [0, 1, 2];
    let mut iter = a.iter().filter(|&&x| x > 1);
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), None);

    // iter.filter(f).next() 等同于 iter.find(f)

    // filter_map() 创建一个既能过滤又能映射的迭代器
    // 返回的迭代器只产生所提供的闭包返回 Some(value) 的值
    let a = ["1", "two", "Nan", "four", "5"];

    let mut iter = a.iter().filter_map(|s| s.parse().ok());
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), Some(5));
    assert_eq!(iter.next(), None);

    // 下面是相同的示例，但使用了filter和map
    let a = ["1", "two", "Nan", "four", "5"];
    let mut iter = a
        .iter()
        .map(|s| s.parse())
        .filter(|s| s.is_ok())
        .map(|s| s.unwrap());

    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), Some(5));
    assert_eq!(iter.next(), None);

    // enumerate() 创建一个迭代器，给出当前迭代次数和下一个值
    // 返回的迭代器产生成对的 (i，val)，其中 i 是当前的迭代索引，val 是迭代器返回的值
    let a = ['a', 'b', 'c'];
    let mut iter = a.iter().enumerate();

    assert_eq!(iter.next(), Some((0, &'a')));
    assert_eq!(iter.next(), Some((1, &'b')));
    assert_eq!(iter.next(), Some((2, &'c')));
    assert_eq!(iter.next(), None);

    // peekable() 创建一个迭代器，该迭代器可以使用 peek 和 peek_mut 方法查看迭代器的下一个元素，而无需消耗它
    let xs = [1, 2, 3];
    let mut iter = xs.iter().peekable();

    // peek() 让我们看到未来
    assert_eq!(iter.peek(), Some(&&1));
    assert_eq!(iter.next(), Some(&1));

    assert_eq!(iter.next(), Some(&2));

    // 我们可以多次 peek()，但迭代器不会前进
    assert_eq!(iter.peek(), Some(&&3));
    assert_eq!(iter.peek(), Some(&&3));

    assert_eq!(iter.next(), Some(&3));

    // 在迭代器结束后，peek() 也会结束
    assert_eq!(iter.peek(), None);
    assert_eq!(iter.next(), None);

    // 使用 peek_mut，在不推进迭代器的情况下突变下一个项目
    let xs = [1, 2, 3];
    let mut iter = xs.iter().peekable();

    assert_eq!(iter.peek_mut(), Some(&mut &1));
    assert_eq!(iter.peek_mut(), Some(&mut &1));
    assert_eq!(iter.next(), Some(&1));

    if let Some(p) = iter.peek_mut() {
        assert_eq!(*p, &2);
        *p = &1000;
    }

    assert_eq!(iter.collect::<Vec<_>>(), vec![&1000, &3]);

    // skip_while() 创建一个迭代器，根据 闭包 跳过元素
    // skip_while() 将一个闭包作为参数。 它会在迭代器的每个元素上调用这个闭包，并忽略元素，直到返回 false
    // 返回 false 后，skip_while() 的工作就结束了，剩下的元素将被输出
    let a = [-1i32, 0, 1];
    let mut iter = a.iter().skip_while(|x| x.is_negative());

    assert_eq!(iter.next(), Some(&0));
    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), None);

    let a = [-1, 0, 1];
    let mut iter = a.iter().skip_while(|x| **x < 0);

    assert_eq!(iter.next(), Some(&0));
    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), None);

    // 初始错误后停止
    let a = [-1, 0, 1, -2];
    let mut iter = a.iter().skip_while(|x| **x < 0);

    assert_eq!(iter.next(), Some(&0));
    assert_eq!(iter.next(), Some(&1));

    // 虽然这将是 false，但由于我们已经得到了 false，所以不再使用 skip_while()。
    assert_eq!(iter.next(), Some(&-2));

    assert_eq!(iter.next(), None);

    // take_while() 创建一个迭代器，根据 闭包 生成元素
    // take_while() 将一个闭包作为参数。 它会在迭代器的每个元素上调用该闭包，并在返回 true 时产生元素
    // 返回 false 后，take_while() 的工作就结束了，其余元素将被忽略

    let a = [-1_i32, 0, 1];
    let mut iter = a.iter().take_while(|x| x.is_negative());

    assert_eq!(iter.next(), Some(&-1));
    assert_eq!(iter.next(), None);

    let a = [-1, 0, 1];
    let mut iter = a.iter().take_while(|x| **x < 0);

    assert_eq!(iter.next(), Some(&-1));
    assert_eq!(iter.next(), None);

    // 初始错误后停止
    let a = [-1, 0, 1, -2];
    let mut iter = a.iter().take_while(|x| **x < 0);

    assert_eq!(iter.next(), Some(&-1));

    // 我们有更多的元素小于零，但由于我们已经得到了一个 false，因此不再使用 take_while()
    assert_eq!(iter.next(), None);

    // 因为 take_while()需要查看值以确定是否应将其包含在内，所以消耗迭代器会看到它被移除了
    let a = [1, 2, 3, 4];
    let mut iter = a.iter();

    let result = iter
        .by_ref()
        .take_while(|n| **n != 3)
        .cloned()
        .collect::<Vec<i32>>();

    assert_eq!(result, &[1, 2]);

    let result = iter.cloned().collect::<Vec<i32>>();

    assert_eq!(result, &[4]);
    // 3 已经不存在了，因为它被消耗了，以确定迭代是否应该停止，但并没有被放回迭代器中

    // !!!!!
    // skip_while() 第一个返回false的元素 返回 所有后面的剩余元素
    // take_while() 第一个返回true的元素  返回 所有前面的元素
    // map_while()  第一个返回None的元素  返回 所有前面的元素

    // map_while() 创建一个迭代器，既能根据 闭包 生成元素，又能映射
    // map_while() 将一个闭包作为参数。 它会在迭代器的每个元素上调用这个闭包，并在返回 Some(_) 的同时产生元素

    let a = [-1_i32, 4, 0, 1];
    let mut iter = a.iter().map_while(|x| 16_i32.checked_div(*x));

    assert_eq!(iter.next(), Some(-16));
    assert_eq!(iter.next(), Some(4));
    assert_eq!(iter.next(), None);

    // 下面是相同的示例，但使用了 take_while 和 map
    let a = [-1_i32, 4, 0, 1];
    let mut iter = a
        .iter()
        .map(|x| 16_i32.checked_div(*x))
        .take_while(|x| x.is_some())
        .map(|x| x.unwrap());

    assert_eq!(iter.next(), Some(-16));
    assert_eq!(iter.next(), Some(4));
    assert_eq!(iter.next(), None);

    // 初始 None 后停止
    let a = [0, 1, 2, -3, 4, 5, -6];
    let iter = a.iter().map_while(|x| u32::try_from(*x).ok());
    let vec = iter.collect::<Vec<_>>();

    // 我们有更多的元素可以放入 u32 (4, 5) 中，但 `map_while` 返回了 `None` 的 `-3` （因为 `predicate` 返回了 `None`）
    // 而 `collect` 在遇到第一个 `None` 时就停止了
    assert_eq!(vec, vec![0, 1, 2]);

    // 因为 map_while()需要查看值以确定是否应将其包含在内，所以消耗迭代器会发现它已被移除
    let a = [1, 2, -3, 4];
    let mut iter = a.iter();

    let result = iter
        .by_ref()
        .map_while(|n| u32::try_from(*n).ok())
        .collect::<Vec<u32>>();

    assert_eq!(result, &[1, 2]);

    let result = iter.cloned().collect::<Vec<i32>>();
    assert_eq!(result, &[4]);
    // 3不再存在，因为它被消耗以确定迭代是否应该停止，但没有被放回迭代器中

    // 请注意，与 take_while 不同，这个迭代器没有融合。
    // 此外，迭代器在返回第一个 None 后会返回什么也没有明确说明。 如果需要融合迭代器，请使用 fuse。

    // skip() 创建一个迭代器，跳过前 n 个元素
    // skip(n)跳过元素，直到跳过 n 个元素或到达迭代器的末端（以先发生者为准）。 之后，所有剩余的元素都会返回
    //  特别是，如果原始迭代器太短，那么返回的迭代器就是空的

    let a = [1, 2, 3];
    let mut iter = a.iter().skip(2);

    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), None);

    // take() 创建一个迭代器，生成前 n 个元素，如果底层迭代器提前结束，则生成更少的元素
    // take(n) 会产生元素，直到产生 n 个元素或到达迭代器的末尾（以先发生者为准）
    // 如果原始迭代器至少包含 n 个元素，则返回的迭代器是长度为 n 的前缀，
    // 否则返回的迭代器包含原始迭代器的所有（少于 n 个）元素。

    let a = [1, 2, 3];
    let mut iter = a.iter().take(2);

    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), None);

    // take() 经常与无限迭代器一起使用，以使其成为有限迭代器
    let mut iter = (0..).take(3);

    assert_eq!(iter.next(), Some(0));
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), None);

    // 如果可用的元素少于 n 个，则 take 会将自身限制在底层迭代器的大小范围内
    let v = [1, 2];
    let mut iter = v.into_iter().take(5);

    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), None);

    // scan() 迭代器适配器，与fold一样，它保存内部状态，但与fold不同的是，它产生一个新的迭代器
    // scan() 有两个参数：一个是作为内部状态种子的初始值，另一个是包含两个参数的闭包，
    // 第一个参数是内部状态的可变引用，第二个参数是迭代器元素。 闭包可以赋值给内部状态，以便在迭代之间共享状态
    let a = [1, 2, 3, 4];

    let mut iter = a.iter().scan(1, |state, &x| {
        // 将状态变量 state 乘以当前元素 x，然后更新状态变量
        *state = *state * x;

        if *state > 6 {
            return None;
        }

        Some(-*state)
    });

    assert_eq!(iter.next(), Some(-1));
    assert_eq!(iter.next(), Some(-2));
    assert_eq!(iter.next(), Some(-6));
    assert_eq!(iter.next(), None);

    // flat_map() 创建一个迭代器，其工作方式与 map 类似，但会将嵌套结构扁平化
    // 映射适配器非常有用，但仅限于闭包参数产生值的情况。 如果它产生的是迭代器，就多了一层间接
    // 你可以把 flat_map(f)看作是映射的语义等价物，然后像 map(f).flatten() 那样进行扁平化
    // 关于 flat_map()的另一种思路：map 的闭包为每个元素返回一个项，而 flat_map() 的闭包为每个元素返回一个迭代器

    let words = ["alpha", "beta", "gamma"];
    // chars() 返回一个 迭代器
    let merged = words.iter().flat_map(|s| s.chars()).collect::<String>();

    assert_eq!(merged, "alphabetagamma");

    // flatten() 创建一个迭代器，对嵌套结构进行扁平化处理
    // 当您有一个迭代器的迭代器，或者有一个可以转化为迭代器的迭代器，而您又想去掉一级间接时，这个功能非常有用
    let data = vec![vec![1, 2, 3, 4], vec![5, 6]];
    let flattened = data.into_iter().flatten().collect::<Vec<u8>>();
    assert_eq!(flattened, vec![1, 2, 3, 4, 5, 6]);

    // Mapping and then flattening
    let words = ["alpha", "beta", "gamma"];

    // chars() 返回一个迭代器
    let merged: String = words.iter().map(|s| s.chars()).flatten().collect();
    assert_eq!(merged, "alphabetagamma");

    // 扁平化适用于任何 IntoIterator 类型，包括Option和Result
    let options = vec![Some(123), Some(321), None, Some(231)];
    let flattened_options = options.into_iter().flatten().collect::<Vec<_>>();
    assert_eq!(flattened_options, vec![123, 321, 231]);

    let results = vec![Ok(123), Ok(321), Err(456), Ok(231)];
    let flattened_results = results.into_iter().flatten().collect::<Vec<_>>();
    assert_eq!(flattened_results, vec![123, 321, 231]);

    // Flattening 每次只移除一级嵌套
    let d3 = [[[1, 2], [3, 4]], [[5, 6], [7, 8]]];
    let d2 = d3.iter().flatten().collect::<Vec<_>>();
    assert_eq!(d2, [&[1, 2], &[3, 4], &[5, 6], &[7, 8]]);

    let d1 = d2.into_iter().flatten().collect::<Vec<_>>();
    assert_eq!(d1, [&1, &2, &3, &4, &5, &6, &7, &8]);

    // fuse() 创建一个迭代器，在第一个 None 之后结束
    // fuse() 对迭代器进行了调整，确保在返回 None 后，迭代器将永远返回 None
    let mut iter = Alternate { state: 0 };

    assert_eq!(iter.next(), Some(0));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), None);

    let mut iter = iter.fuse();

    assert_eq!(iter.next(), Some(4));
    assert_eq!(iter.next(), None);

    // 它将始终返回 `None`
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);

    // inspect() 对迭代器中的每个元素进行处理
    // 在使用迭代器时，您通常会将多个迭代器链在一起
    // 在处理此类代码时，您可能想查看管道中各个部分的情况。 为此，请调用 inspect()

    let a = [1, 4, 2, 3];
    let sum = a
        .iter()
        .cloned()
        .filter(|x| x % 2 == 0)
        .fold(0, |sum, i| sum + i);

    println!("{}", sum);

    let sum = a
        .iter()
        .cloned()
        .inspect(|x| println!("about to filter: {}", x))
        .filter(|x| x % 2 == 0)
        .inspect(|x| println!("made it through filter: {}", x))
        .fold(0, |sum, i| sum + i);

    println!("{}", sum);

    // 在丢弃错误前记录错误
    let lines = ["1", "2", "a"];

    let sum: i32 = lines
        .iter()
        .map(|line| line.parse::<i32>())
        .inspect(|num| {
            if let Err(ref e) = *num {
                println!("Parsing error: {}", e);
            }
        })
        .filter_map(Result::ok)
        .sum();

    println!("Sum: {}", sum);

    // by_ref() 借用迭代器，而不是消耗它
    // 这对于在应用迭代器适配器的同时保留原始迭代器的所有权非常有用
    let mut words = ["hello", "world", "of", "Rust"].into_iter();

    let hello_world = words.by_ref().take(2).collect::<Vec<_>>();
    assert_eq!(hello_world, vec!["hello", "world"]);

    // 我们之所以能这样做，是因为我们之前使用了 `by_ref`
    let of_rust = words.collect::<Vec<_>>();
    assert_eq!(of_rust, vec!["of", "Rust"]);

    // collect() 将迭代器转换为集合
    // collect() 可以接收任何可迭代的数据，并将其转化为相关的集合。 这是标准库中功能更强大的方法之一，可用于多种场合

    // 由于 collect() 非常通用，可能会导致类型推断方面的问题。
    // 因此，collect() 是你为数不多能看到  称为 "涡轮鱼 "的语法 ::<>。 这有助于推理算法具体了解您要将哪个集合收集
    let a = [1, 2, 3];

    let doubled = a.iter().map(|&x| x * 2).collect::<Vec<i32>>();

    assert_eq!(vec![2, 4, 6], doubled);

    // 请注意，我们需要添加 ::Vec<i32>。 这是因为我们可以收集到一个 VecDeque<T> 来代替
    let a = [1, 2, 3];

    let doubled = a.iter().map(|&x| x * 2).collect::<VecDeque<i32>>();

    assert_eq!(2, doubled[0]);
    assert_eq!(4, doubled[1]);
    assert_eq!(6, doubled[2]);

    let a = [1, 2, 3];

    let doubled = a.iter().map(|x| x * 2).collect::<Vec<i32>>();

    assert_eq!(vec![2, 4, 6], doubled);

    // 由于 collect() 只关心收集到的内容，因此您仍然可以使用部分类型提示 _，并使用 turbofish
    let a = [1, 2, 3];

    let doubled = a.iter().map(|x| x * 2).collect::<Vec<_>>();

    assert_eq!(vec![2, 4, 6], doubled);

    // 使用 collect() 创建字符串
    let chars = ['g', 'd', 'k', 'k', 'n'];

    let hello = chars
        .iter()
        .map(|&x| x as u8)
        .map(|x| (x + 1) as char)
        .collect::<String>();

    assert_eq!("hello", hello);

    // 如果您有一个 Result<T, E> 列表，您可以使用 collect() 查看是否有任何结果失败
    // 如果所有的元素都是Ok，那么结果将是Ok(Vec<T>)，其中Vec<T>包含所有成功的值
    // 如果遇到任何一个Err，则collect会立即返回这个Err，而不会继续处理后续的元素
    let results = [Ok(1), Err("nope"), Ok(3), Err("bad")];
    let result = results.iter().cloned().collect::<Result<Vec<_>, &str>>();
    assert_eq!(Err("nope"), result);

    let results = [Ok(1), Ok(3)];

    let result = results.iter().cloned().collect::<Result<Vec<_>, &str>>();
    assert_eq!(Ok(vec![1, 3]), result);

    // partition() 消耗一个迭代器，从中创建两个集合
    // 传给 partition() 的谓词可以返回 true，也可以返回 false
    // partition() 返回一对，即返回 true 的所有元素和返回 false 的所有元素

    let a = [1, 2, 3];

    let (even, odd): (Vec<_>, Vec<_>) = a.into_iter().partition(|n| n % 2 == 0);

    assert_eq!(even, vec![2]);
    assert_eq!(odd, vec![1, 3]);

    // try_fold() 迭代器方法，只要函数成功返回，就会应用该函数，产生一个单一的最终值
    // try_fold() 包含两个参数：一个初始值和一个包含两个参数的闭包：一个 "累加器 "和一个元素
    // 闭包要么成功返回累加器在下一次迭代中的值，要么失败返回错误值，错误值会立即传回调用者（短路

    let a = [1, 2, 3];

    let sum = a.iter().try_fold(0_i8, |acc, &x| acc.checked_add(x));

    assert_eq!(sum, Some(6));

    // Short-circuiting  短路
    let a = [10, 20, 30, 100, 40, 50];
    let mut it = a.iter();

    // 由于合会大于 i8最大值 会报错 返回None
    let sum = it.try_fold(0_i8, |acc, &x| acc.checked_add(x));
    assert_eq!(sum, None);

    // 因为它短路了，所以剩余的元素仍然可以通过迭代器获得。
    assert_eq!(it.len(), 2);
    assert_eq!(it.next(), Some(&40));
    assert_eq!(it.next(), Some(&50));
    assert_eq!(it.next(), None);

    // 虽然不能从闭包中断，但 ControlFlow 类型可以实现类似的想法
    let triangular = (1..30).try_fold(0_i8, |prev, x| {
        if let Some(next) = prev.checked_add(x) {
            ControlFlow::Continue(next)
        } else {
            ControlFlow::Break(prev)
        }
    });
    assert_eq!(triangular, ControlFlow::Break(120));

    let triangular = (1..30).try_fold(0_u64, |prev, x| {
        if let Some(next) = prev.checked_add(x) {
            ControlFlow::Continue(next)
        } else {
            ControlFlow::Break(prev)
        }
    });

    assert_eq!(triangular, ControlFlow::Continue(435));

    // try_for_each() 一个迭代器方法，对迭代器中的每个项目应用一个易错函数，在第一个错误时停止并返回该错误
    // 这也可以看作是 for_each() 的易错形式或 try_fold() 的无状态版本。
    let data = ["no_tea.txt", "stale_bread.json", "torrential_rain.png"];

    let res = data.iter().try_for_each(|x| writeln!(stdout(), "{}", x));
    assert!(res.is_ok());

    let mut it = data.iter().cloned();
    let res = it.try_for_each(|x| fs::rename(x, Path::new(x).with_extension("old")));
    assert!(res.is_err());

    assert_eq!(it.next(), Some("stale_bread.json"));

    // ControlFlow 类型可与该方法一起用于在普通循环中使用 break 和 continue 的情况
    let r = (2..100).try_for_each(|x| {
        if 323 % x == 0 {
            return ControlFlow::Break(x);
        }
        ControlFlow::Continue(())
    });
    assert_eq!(r, ControlFlow::Break(17));

    // fold() 通过运算将每个元素折叠成累加器，返回最终结果
    // fold() 包含两个参数：一个初始值和一个包含两个参数的闭包：一个 "累加器 "和一个元素。 闭包返回累加器下一次迭代时的值
    // 初始值是累加器在第一次调用时的值
    let a = [1, 2, 3];
    let sum = a.iter().fold(0, |acc, x| acc + x);
    assert_eq!(sum, 6);

    // 构建一个字符串
    let numbers = [1, 2, 3, 4, 5];

    let zero = "0".to_string();

    let result = numbers
        .iter()
        .fold(zero, |acc, &x| format!("({} + {})", acc, x));
    assert_eq!(result, "(((((0 + 1) + 2) + 3) + 4) + 5)");

    // 对于不经常使用迭代器的人来说，使用包含一系列内容的 for 循环来建立结果是很常见的。 这些可以变成 fold()
    let numbers = [1, 2, 3, 4, 5];

    let mut result = 0;

    // for 循环
    for i in &numbers {
        result = result + i;
    }

    // fold
    let result2 = numbers.iter().fold(0, |acc, &x| acc + x);

    assert_eq!(result, result2);

    // reduce() 通过重复应用还原操作，将元素还原为单一元素
    // 如果迭代器为空，则返回 None；否则返回还原结果
    let reduced = (1..10).reduce(|acc, e| acc + e).unwrap();
    assert_eq!(reduced, 45);

    let folded = (1..10).fold(0, |acc, e| acc + e);
    assert_eq!(reduced, folded);

    // 全部为true 就是true 否则就是 false
    // all()  测试迭代器中的每个元素是否都符合闭包   空迭代器返回 true
    // all() 使用一个返回 true 或 false 的闭包。 它将此闭包应用于迭代器的每个元素
    // 如果它们都返回 true，则 all() 也返回 true  如果其中任何一个返回 false，则返回 false
    // all() 是短路的；换句话说，一旦发现 false，它就会停止处理，因为无论发生什么，结果都是 false
    let a = [1, 2, 3];
    assert!(a.iter().all(|&x| x > 0));
    assert!(!a.iter().all(|&x| x > 2));

    let a = [1, 2, 3];

    let mut iter = a.iter();
    assert!(!iter.all(|&x| x != 2));

    // 我们仍然可以使用 `iter`，因为有更多的元素
    assert_eq!(iter.next(), Some(&3));

    // 有一个true 就是true  全部是false 才是false
    // any() 测试迭代器中的任何元素是否与闭包匹配   空迭代器返回 false
    // any() 使用一个返回 true 或 false 的闭包 它将此闭包应用于迭代器的每个元素
    // 如果其中任何元素返回 true，any() 也会应用。 如果所有元素都返回 false，则返回 false
    // any() 是短路的；换句话说，一旦发现 true，它就会停止处理，因为无论发生什么，结果都是 true
    let a = [1, 2, 3];
    assert!(a.iter().any(|&x| x > 0));
    assert!(!a.iter().any(|&x| x > 5));

    let a = [1, 2, 3];
    let mut iter = a.iter();

    assert!(iter.any(|&x| x != 2));
    // 我们仍然可以使用 `iter`，因为有更多的元素
    assert_eq!(iter.next(), Some(&2));

    // 找到元素 直接返回该元素  找不到返回 None
    // find()  搜索迭代器中满足闭包的元素
    // find() 使用一个返回 true 或 false 的闭包。 它将此闭包应用于迭代器的每个元素
    // 如果其中任何元素返回 true，则 find() 返回 Some(element)。 如果所有元素都返回 false，则返回 None
    // find() 是短路的；换句话说，一旦闭包返回 true，它就会停止处理
    // 如果需要元素的索引，请参阅 position()
    let a = [1, 2, 3];
    assert_eq!(a.iter().find(|&&x| x == 2), Some(&2));
    assert_eq!(a.iter().find(|&&x| x == 5), None);

    let a = [1, 2, 3];
    let mut iter = a.iter();
    assert_eq!(iter.find(|&&x| x == 2), Some(&2));

    // 我们仍然可以使用 `iter`，因为有更多的元素
    assert_eq!(iter.next(), Some(&3));

    // find_map() 对迭代器中的元素应用函数，并返回第一个非空结果
    // iter.find_map(f) 等价于 iter.filter_map(f).next()
    let a = ["lol", "NaN", "2", "5"];
    let first_number = a.iter().find_map(|s| s.parse().ok());
    assert_eq!(first_number, Some(2));

    // position() 在迭代器中搜索元素，返回其索引
    // position() 使用一个返回 true 或 false 的闭包。 它将此闭包应用于迭代器的每个元素
    // 如果其中一个元素返回 true，则 position() 返回 Some(index)。 如果所有闭包都返回 false，则返回 None
    let a = [1, 2, 3];
    assert_eq!(a.iter().position(|&x| x == 2), Some(1));
    assert_eq!(a.iter().position(|&x| x == 5), None);

    let a = [1, 2, 3, 4];
    let mut iter = a.iter();
    assert_eq!(iter.position(|&x| x >= 2), Some(1));
    // 我们仍然可以使用 `iter`，因为有更多的元素
    assert_eq!(iter.next(), Some(&3));
    // 返回的索引取决于迭代器状态
    assert_eq!(iter.position(|&x| x == 4), Some(0));

    // rposition() 从右侧搜索迭代器中的元素，返回其索引
    // rposition() 使用一个返回 true 或 false 的闭包。
    // 如果其中一个返回 true，则 rposition() 返回 Some(index)。 如果所有闭包都返回 false，则返回 None
    let a = [1, 2, 3];
    assert_eq!(a.iter().rposition(|&x| x == 3), Some(2));
    assert_eq!(a.iter().rposition(|&x| x == 5), None);

    let a = [-1, 2, 3, 4];
    let mut iter = a.iter();
    assert_eq!(iter.rposition(|&x| x >= 2), Some(3));

    // 我们仍然可以使用 `iter`，因为有更多的元素
    assert_eq!(iter.next(), Some(&-1));

    // max() 返回迭代器的最大元素
    // 如果多个元素的最大值相同，则返回最后一个元素。 如果迭代器为空，则返回 None

    assert_eq!(
        [2.4, f32::NAN, 1.3].into_iter().reduce(f32::max).unwrap(),
        2.4
    );

    let a = [1, 2, 3];
    let b: Vec<u32> = Vec::new();
    assert_eq!(a.iter().max(), Some(&3));
    assert_eq!(b.iter().max(), None);

    // min() 返回迭代器的最小元素
    // 如果多个元素的最小值相同，则返回第一个元素。 如果迭代器为空，则返回 None
    assert_eq!(
        [2.4, f32::NAN, 1.3].into_iter().reduce(f32::min).unwrap(),
        1.3
    );

    let a = [1, 2, 3];
    let b: Vec<u32> = Vec::new();
    assert_eq!(a.iter().min(), Some(&1));
    assert_eq!(b.iter().min(), None);

    // max_by_key() 返回指定函数中给出最大值的元素
    // 如果多个元素的最大值相同，则返回最后一个元素。 如果迭代器为空，则返回 None
    let a = [-3_i32, 0, 1, 5, -10];
    assert_eq!(*a.iter().max_by_key(|x| x.abs()).unwrap(), -10);

    // max() 返回与指定比较函数相关的最大值的元素
    // 如果多个元素的最大值相同，则返回最后一个元素。 如果迭代器为空，则返回 None
    let a = [-3_i32, 0, 1, 5, -10];
    assert_eq!(*a.iter().max_by(|x, y| x.cmp(y)).unwrap(), 5);

    // min_by_key() 返回指定函数中给出最小值的元素
    // 如果多个元素的最小值相同，则返回第一个元素。 如果迭代器为空，则返回 None
    let a = [-3_i32, 0, 1, 5, -10];
    assert_eq!(*a.iter().min_by_key(|x| x.abs()).unwrap(), 0);

    // min_by() 返回与指定比较函数相关的最小值的元素
    // 如果多个元素的最小值相同，则返回第一个元素。 如果迭代器为空，则返回 None
    let a = [-3_i32, 0, 1, 5, -10];
    assert_eq!(*a.iter().min_by(|x, y| x.cmp(y)).unwrap(), -10);

    // rev() 反转迭代器的元素
    // 通常，迭代器从左到右迭代。 使用 rev() 后，迭代器将从右向左迭代
    let a = [1, 2, 3];
    let mut iter = a.iter().rev();
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), None);

    // unzipA() 将一对迭代器转换为一对容器
    // unzip() 会消耗整个线对迭代器，产生两个集合：一个是线对左侧元素的集合，另一个是线对右侧元素的集合
    // 从某种意义上说，该功能与zip功能相反

    let a = [(1, 2), (3, 4), (5, 6)];
    let (left, right): (Vec<_>, Vec<_>) = a.iter().cloned().unzip();
    assert_eq!(left, [1, 3, 5]);
    assert_eq!(right, [2, 4, 6]);

    // 也可以一次解压缩多个嵌套图元
    let a = [(1, (2, 3)), (4, (5, 6))];

    let (x, (y, z)): (Vec<_>, (Vec<_>, Vec<_>)) = a.iter().cloned().unzip();
    assert_eq!(x, [1, 4]);
    assert_eq!(y, [2, 5]);
    assert_eq!(z, [3, 6]);

    // copied() 创建一个迭代器，复制其所有元素
    // 当你有一个 &T 的迭代器，但需要一个 T 的迭代器时，这个功能很有用
    let a = [1, 2, 3];
    let v_copied = a.iter().copied().collect::<Vec<_>>();

    // 复制与 .map(|&x| x) 相同
    let v_map = a.iter().map(|&x| x).collect::<Vec<_>>();

    assert_eq!(v_copied, v_map);
    assert_eq!(v_copied, vec![1, 2, 3]);
    assert_eq!(v_map, vec![1, 2, 3]);

    // cloned() 创建一个迭代器，克隆其所有元素
    // 当你有一个 &T 的迭代器，但需要一个 T 的迭代器时，这个功能很有用
    // 无法保证克隆方法会被调用或优化。 因此，代码不应依赖这两种方法
    let a = [1, 2, 3];
    let v_cloned = a.iter().cloned().collect::<Vec<_>>();

    // 克隆与 .map(|&x| x) 相同，适用于整数
    let v_map = a.iter().map(|&x| x).collect::<Vec<_>>();

    assert_eq!(v_cloned, v_map);
    assert_eq!(v_cloned, vec![1, 2, 3]);
    assert_eq!(v_map, vec![1, 2, 3]);

    // 要获得最佳性能，请尽量 最后 克隆
    let a = [vec![0_u8, 1, 2], vec![3, 4], vec![23]];

    // 不要这样做
    let slower = a
        .iter()
        .cloned()
        .filter(|s| s.len() == 1)
        .collect::<Vec<_>>();
    assert_eq!(&[vec![23]], &slower[..]);

    // 最后调用 cloned()
    let faster = a
        .iter()
        .filter(|s| s.len() == 1)
        .cloned()
        .collect::<Vec<_>>();
    assert_eq!(&[vec![23]], &faster[..]);

    // cycle() 无休止地重复迭代器
    // 迭代器不会在 None 处停止，而是会从头开始。 再次迭代后，它又会从头开始。 再开始 再一次 永远如此
    // 请注意，如果原始迭代器为空，那么生成的迭代器也将为空
    let a = [1, 2, 3];
    let mut it = a.iter().cycle();

    assert_eq!(it.next(), Some(&1));
    assert_eq!(it.next(), Some(&2));
    assert_eq!(it.next(), Some(&3));
    assert_eq!(it.next(), Some(&1));
    assert_eq!(it.next(), Some(&2));
    assert_eq!(it.next(), Some(&3));
    assert_eq!(it.next(), Some(&1));

    // sum() 对迭代器中的元素求和    空迭代器返回类型的零
    // 将每个元素相加，并返回结果
    // sum() 可用于对任何实现 Sum 的类型求和，包括 Option 和 Result
    let a = [1, 2, 3];
    let sum: i32 = a.iter().sum();
    assert_eq!(sum, 6);

    // product() 对整个迭代器进行迭代，乘以所有元素  阶乘
    // 空迭代器返回的值类型为
    // product() 可用于对任何实现 Product 的类型进行乘法运算，包括 Option 和 Result
    assert_eq!(factorial(0), 1);
    assert_eq!(factorial(1), 1);
    assert_eq!(factorial(5), 120);

    // cmp() 将该迭代器的元素与另一个迭代器的元素进行比较
    assert_eq!([1].iter().cmp([1].iter()), Ordering::Equal);
    assert_eq!([1].iter().cmp([1, 2].iter()), Ordering::Less);
    assert_eq!([1, 2].iter().cmp([1].iter()), Ordering::Greater);

    // partial_cmp()
    // 在将此迭代器中的 PartialOrd 元素与另一个迭代器中的 PartialOrd 元素进行比较。
    // 这种比较类似于短路评估，在不比较其余元素的情况下返回结果。 一旦可以确定一个顺序，计算就会停止，并返回一个结果
    assert_eq!([1.].iter().partial_cmp([1.].iter()), Some(Ordering::Equal));
    assert_eq!(
        [1.].iter().partial_cmp([1., 2.].iter()),
        Some(Ordering::Less)
    );
    assert_eq!(
        [1., 2.].iter().partial_cmp([1.].iter()),
        Some(Ordering::Greater)
    );

    // 对于浮点数，NaN 没有总数阶，比较时结果为 None
    assert_eq!([f64::NAN].iter().partial_cmp([1.].iter()), None);

    // 结果由评估顺序决定
    assert_eq!(
        [1.0, f64::NAN].iter().partial_cmp([2.0, f64::NAN].iter()),
        Some(Ordering::Less)
    );
    assert_eq!(
        [2.0, f64::NAN].iter().partial_cmp([1.0, f64::NAN].iter()),
        Some(Ordering::Greater)
    );
    assert_eq!(
        [f64::NAN, 1.0].iter().partial_cmp([f64::NAN, 2.0].iter()),
        None
    );

    // eq() 确定此迭代器中的元素是否等于另一个迭代器中的元素
    assert_eq!([1].iter().eq([1].iter()), true);
    assert_eq!([1].iter().eq([1, 2].iter()), false);

    // ne() 确定此迭代器中的元素是否与另一个迭代器中的元素不相等
    assert_eq!([1].iter().ne([1].iter()), false);
    assert_eq!([1].iter().ne([1, 2].iter()), true);

    // lt() 确定该迭代器中的元素在词法上是否小于另一个迭代器中的元素
    assert_eq!([1].iter().lt([1].iter()), false);
    assert_eq!([1].iter().lt([1, 2].iter()), true);
    assert_eq!([1, 2].iter().lt([1].iter()), false);
    assert_eq!([1, 2].iter().lt([1, 2].iter()), false);

    // le() 确定该迭代器中的元素在词法上是否小于或等于另一个迭代器中的元素
    assert_eq!([1].iter().le([1].iter()), true);
    assert_eq!([1].iter().le([1, 2].iter()), true);
    assert_eq!([1, 2].iter().le([1].iter()), false);
    assert_eq!([1, 2].iter().le([1, 2].iter()), true);

    // gt() 确定此迭代器中的元素在词法上是否大于另一个迭代器中的元素
    assert_eq!([1].iter().gt([1].iter()), false);
    assert_eq!([1].iter().gt([1, 2].iter()), false);
    assert_eq!([1, 2].iter().gt([1].iter()), true);
    assert_eq!([1, 2].iter().gt([1, 2].iter()), false);

    // ge() 确定此迭代器中的元素在词法上是否大于或等于另一个迭代器中的元素
    assert_eq!([1].iter().ge([1].iter()), true);
    assert_eq!([1].iter().ge([1, 2].iter()), false);
    assert_eq!([1, 2].iter().ge([1].iter()), true);
    assert_eq!([1, 2].iter().ge([1, 2].iter()), true);
}

fn factorial(n: u32) -> u32 {
    (1..=n).product()
}

struct Alternate {
    state: i32,
}

impl Iterator for Alternate {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        let val = self.state;
        self.state = self.state + 1;

        if val % 2 == 0 {
            Some(val)
        } else {
            None
        }
    }
}
