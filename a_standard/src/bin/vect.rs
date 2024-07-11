#![feature(
    allocator_api,
    vec_into_raw_parts,
    vec_push_within_capacity,
    vec_pop_if,
    vec_split_at_spare,
    const_slice_flatten,
    extract_if,
    sort_floats,
    slice_swap_unchecked,
    slice_as_chunks
)]

use std::alloc::{alloc, AllocError, Allocator, Global, Layout, System};
use std::cell::Cell;
use std::collections::TryReserveError;
use std::io::{Read, Write};
use std::ops::Add;
use std::ptr;
use std::{io, mem};

fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);

    assert_eq!(vec.len(), 2);
    assert_eq!(vec[0], 1);

    assert_eq!(vec.pop(), Some(2));
    assert_eq!(vec.len(), 1);

    vec[0] = 7;
    assert_eq!(vec[0], 7);

    vec.extend([1, 2, 3]);

    for x in &vec {
        print!("{}", x);
    }

    assert_eq!(vec, [7, 1, 2, 3]);

    // 提供 vec! 宏是为了方便初始化：
    let mut vec1 = vec![1, 2, 3];
    vec1.push(4);
    let vec2 = Vec::from([1, 2, 3, 4]);
    assert_eq!(vec1, vec2);

    // 还可以用给定值初始化 Vec<T> 的每个元素。
    // 这可能比分别执行分配和初始化更有效，尤其是在初始化一个零向量时：
    let vec = vec![0; 5];
    assert_eq!(vec, [0, 0, 0, 0, 0]);

    // 下面的效果与上面一样  效率低一点
    let mut vec = Vec::with_capacity(5);
    vec.resize(5, 0);
    assert_eq!(vec, [0, 0, 0, 0, 0]);

    println!();

    // 使用 Vec<T> 作为高效堆栈
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    // Vec 类型允许通过索引访问值，因为它实现了索引特性
    let v = vec![0, 2, 4, 6];
    println!("{}", v[1]);
    // 但是要小心：如果您试图访问 Vec 中没有的索引，您的软件就会崩溃！您不能这样做
    // println!("{}", v[6]);

    // Slicing     Vec 可以是可变的。另一方面，片段是只读对象。要获取片段，请使用 & 示例
    let v = vec![0, 1];
    read_slice(&v);
    // 也可以这样写
    let _u: &[usize] = &v;
    let _u: &[_] = &v;

    // 构造一个新的 空Vec<T>。
    // 在元素被推入之前，该向量不会分配内存
    let mut _vec: Vec<i32> = Vec::new();

    // 构造一个新的空 Vec<T>，并指定容量
    // 在不重新分配的情况下，向量将至少能容纳容量元素。该方法允许分配的元素数量超过容量。
    // 如果容量为 0，向量将不会分配。
    // 如果新容量超过 isize::MAX 字节，则系统崩溃。
    let mut vec: Vec<i32> = Vec::with_capacity(10);

    // Vec中没有项目，尽管它可以容纳更多项目
    assert_eq!(vec.len(), 0);
    assert_eq!(vec.capacity(), 10);

    // 这些都没有触发重新分配  因为Vec容量足够
    for i in 0..10 {
        vec.push(i);
    }
    assert_eq!(vec.len(), 10);
    assert_eq!(vec.capacity(), 10);

    // 但这可能会使向量重新分配
    vec.push(10);
    assert_eq!(vec.len(), 11);
    assert!(vec.capacity() >= 11);
    println!("{}", vec.capacity());

    // 零大小类型的向量总是超量分配，因为不需要
    // 无需分配
    let vec_units = Vec::<()>::with_capacity(10);
    assert_eq!(vec_units.capacity(), usize::MAX);

    // from_raw_parts() 通过指针、长度和容量直接创建 Vec<T>。
    let v = vec![1, 2, 3];
    // 防止运行 `v` 的析构函数，这样我们就能完全控制
    // 分配。
    let mut v = mem::ManuallyDrop::new(v);

    // 调出有关 `v` 的各种重要信息
    let p = v.as_mut_ptr();
    let len = v.len();
    let cap = v.capacity();

    unsafe {
        // 用 4、5、6 重写内存
        for i in 0..len {
            ptr::write(p.add(i), 4 + i);
        }

        // 将所有内容重新组合成一个 Vec
        let rebuilt = Vec::from_raw_parts(p, len, cap);
        assert_eq!(rebuilt, [4, 5, 6]);
    }

    // 使用已在别处分配的内存
    let layout = Layout::array::<u32>(16).expect("overflow cannot happen");

    let vec = unsafe {
        let mem = alloc(layout).cast::<u32>();
        if mem.is_null() {
            return;
        }

        mem.write(1_000_000);

        Vec::from_raw_parts(mem, 1, 16)
    };

    assert_eq!(vec, &[1_000_000]);
    assert_eq!(vec.capacity(), 16);

    // new_in(alloc: A) 构造一个新的空 Vec<T,A>  夜间版方法
    // 在将元素推入向量之前，向量不会分配。
    let mut vec: Vec<i32, _> = Vec::new_in(System);

    //  with_capacity_in(capacity: usize, alloc: A) 夜间版方法
    // 使用提供的分配器构造一个新的空 Vec<T，A>，其容量至少为指定值。
    let mut vec: Vec<i32, System> = Vec::with_capacity_in(10, System);
    assert_eq!(vec.len(), 0);
    assert!(vec.capacity() >= 10);

    for i in 0..10 {
        vec.push(i);
    }
    assert_eq!(vec.len(), 10);
    assert!(vec.capacity() >= 10);

    vec.push(10);
    assert_eq!(vec.len(), 11);
    assert!(vec.capacity() >= 11);

    let vec_units = Vec::<(), System>::with_capacity_in(10, System);
    assert_eq!(vec_units.capacity(), usize::MAX);

    // try_with_capacity()
    // try_with_capacity_in()

    // from_raw_parts_in() 通过指针、长度、容量和分配器直接创建 Vec<T，A>  夜间版方法
    let mut v = Vec::with_capacity_in(3, System);
    v.push(1);
    v.push(2);
    v.push(3);

    // 防止运行 `v` 的析构函数，这样我们就能完全控制
    // 分配
    let mut v = mem::ManuallyDrop::new(v);

    // 提取关于 `v` 的各种重要信息
    let p = v.as_mut_ptr();
    let len = v.len();
    let cap = v.capacity();
    let alloc = v.allocator();

    unsafe {
        for i in 0..len {
            ptr::write(p.add(i), 4 + i);
        }

        // 将所有内容重新组合成一个 Vec
        let rebuilt = Vec::from_raw_parts_in(p, len, cap, alloc.clone());
        assert_eq!(rebuilt, [4, 5, 6]);
    }
    // 使用其他地方分配的内存
    let layout = Layout::array::<u32>(16).expect("overflow cannot happen");

    let vec = unsafe {
        let mem = match Global.allocate(layout) {
            Ok(mem) => mem.cast::<u32>().as_ptr(),
            Err(AllocError) => return,
        };

        mem.write(1_000_000);

        Vec::from_raw_parts_in(mem, 1, 16, Global)
    };

    assert_eq!(vec, &[1_000_000]);
    assert_eq!(vec.capacity(), 16);

    // into_raw_parts() 将 Vec<T> 分解为原始组件：(指针、长度、容量）  夜间版方法
    // 调用此函数后，调用者将对之前由 Vec 管理的内存负责。
    // 唯一的办法是使用 from_raw_parts 函数将原始指针、长度和容量转换回 Vec，然后让析构函数执行清理。
    let v = vec![-1, 0, 1];
    let (ptr, len, cap) = v.into_raw_parts();

    let rebuilt = unsafe {
        // 我们现在可以对组件进行修改，例如
        // 将原始指针转换为兼容类型
        let ptr = ptr as *mut u32;

        Vec::from_raw_parts(ptr, len, cap)
    };
    assert_eq!(rebuilt, [4294967295, 0, 1]);

    // into_raw_parts_with_alloc()  将 Vec<T> 分解为原始组件：(指针、长度、容量、分配器） 夜间版方法
    // 调用此函数后，调用者将对之前由 Vec 管理的内存负责。
    // 唯一的办法是使用 from_raw_parts_in 函数将原始指针、长度和容量转换回 Vec，并允许析构函数执行清理。
    let mut v: Vec<i32, System> = Vec::new_in(System);

    v.push(-1);
    v.push(0);
    v.push(1);

    let (ptr, len, cap, alloc) = v.into_raw_parts_with_alloc();

    let rebuilt = unsafe {
        // 我们现在可以对组件进行修改，例如
        // 将原始指针转换为兼容类型。
        let ptr = ptr as *mut u32;
        Vec::from_raw_parts_in(ptr, len, cap, alloc)
    };
    assert_eq!(rebuilt, [4294967295, 0, 1]);

    // capacity() 返回Vec在不重新分配的情况下可容纳的元素总数。
    let mut vec = Vec::with_capacity(10);
    vec.push(43);
    println!("capacity = {}", vec.capacity());
    assert!(vec.capacity() >= 10);

    // reserve() 为 Vec 预分配至少 additional 个元素的空间。
    // 可能会多分配一些空间 以便将来能减少重新分配的次数。
    let mut vec = vec![1];
    vec.reserve(10);
    println!("capacity = {}", vec.capacity());
    assert!(vec.capacity() >= 11);

    // reserve_exact() 为 Vec 预分配刚好 additional 个元素的空间。
    // 仅分配指定的空间，不会额外分配。
    let mut vec = vec![1];
    vec.reserve_exact(10);
    println!("capacity = {}", vec.capacity());
    assert!(vec.capacity() >= 11);

    // try_reserve() 尝试为 Vec 预分配至少 additional 个元素的空间。
    // 如果分配失败，返回一个 Result，而不是导致程序 panic。
    // 适用场景：当你需要保证程序在内存分配失败时不会崩溃，而是能够优雅地处理错误时使用。
    let vec = [1, 2, 3, 4, 5];
    println!("{:?}", process_data(&vec));

    // try_reserve_exact() 尝试为 Vec 预分配刚好 additional 个元素的空间。
    // 特点：如果分配失败，返回一个 Result，而不是导致程序 panic
    let vec = [2, 4, 6, 8, 10];
    println!("{:?}", process_data2(&vec));

    // shrink_to_fit() 尽可能缩小 Vec 的容量
    let mut vec = Vec::with_capacity(10);
    vec.extend([1, 2, 3]);
    println!("容量缩小前={}", vec.capacity());
    assert!(vec.capacity() >= 10);
    vec.shrink_to_fit();
    println!("容量缩小后={}", vec.capacity());
    assert!(vec.capacity() >= 3);

    // shrink_to() 以下限缩减向量的容量。
    // 容量至少与长度和提供的值一样大。
    // 如果当前容量小于下限，则不会执行此操作。
    let mut vec = Vec::with_capacity(10);
    vec.extend([1, 2, 3]);
    println!("容量缩小前={}", vec.capacity());
    assert!(vec.capacity() >= 10);

    vec.shrink_to(4);
    println!("缩小到指定容量={}", vec.capacity());
    assert!(vec.capacity() >= 4);

    vec.shrink_to(0);
    println!("缩小到指定容量={}", vec.capacity());
    assert!(vec.capacity() >= 3);

    // into_boxed_slice()  将Vec转换为 Box<[T]>。
    // 在转换之前，该方法会像 shrink_to_fit 一样丢弃多余的容量
    let vec = vec![1, 2, 3];
    let slice = vec.into_boxed_slice();
    println!("box={:?}", slice);

    // 剩余容量将被移除
    let mut vec = Vec::with_capacity(10);
    vec.extend([1, 2, 3]);

    assert!(vec.capacity() >= 10);
    let slice = vec.into_boxed_slice();
    assert_eq!(slice.into_vec().capacity(), 3);

    //  truncate() 缩短Vec，保留前 len 元素，去掉其余元素。
    // 如果 len 大于或等于Vec的当前长度，则不会产生任何影响。
    // drain 方法可以模拟 truncate，但会返回而不是丢弃多余的元素。
    // 请注意，这种方法对Vec的分配容量没有影响

    // 将五个元素的Vec截断为两元素
    let mut vec = vec![1, 2, 3, 4, 5];
    vec.truncate(2);
    println!("截断后={:?}", vec);
    assert_eq!(vec, [1, 2]);

    // 当截断 len 大于Vec的当前长度时，不会发生截断：
    let mut vec = vec![1, 2, 3];
    vec.truncate(8);
    println!("截断后={:?}", vec);
    assert_eq!(vec, [1, 2, 3]);

    // 当截断 len == 0 时进行截断，相当于调用清除方法。
    let mut vec = vec![1, 2, 3];
    vec.truncate(0);
    println!("截断后={:?}", vec);
    assert_eq!(vec, []);

    // as_slice() 提取包含整个Vec的片段。
    // 等价于 &s[...]
    let buffer = vec![1, 2, 3, 5, 8];
    io::sink().write(buffer.as_slice()).unwrap();

    // as_mut_slice() 提取整个Vec的一个可变片段。
    // 等价于 &mut s[...]
    let mut buffer = vec![0; 3];
    io::repeat(0b101).read_exact(buffer.as_mut_slice()).unwrap();

    // as_ptr() 返回指向Vec缓冲区的原始指针，如果Vec没有分配，则返回对零大小读取有效的悬空原始指针
    let vec = vec![1, 2, 4];
    let vec_ptr = vec.as_ptr();

    unsafe {
        for i in 0..vec.len() {
            println!("{}, {}", i, 1 << i);
            assert_eq!(*vec_ptr.add(i), 1 << i);
        }
    }

    // 由于有别名保证，以下代码是合法的
    unsafe {
        let mut v = vec![0, 1, 2];
        let ptr1 = v.as_ptr();
        let _ = ptr1.read();
        let ptr2 = v.as_mut_ptr().offset(2);
        ptr2.write(2);

        // 值得注意的是，对 `ptr2` 的写入并没有** 使 `ptr1` 无效。
        // 因为它改变了另一个元素：
        let _ = ptr1.read();
    }

    // as_mut_ptr() 返回指向Vec缓冲区的不安全可变指针，如果Vec没有分配，则返回对零大小读取有效的悬空原始指针。
    let size = 4;
    let mut x: Vec<i32> = Vec::with_capacity(size);
    let x_ptr = x.as_mut_ptr();

    // 通过原始指针写入初始化元素，然后设置长度
    unsafe {
        for i in 0..size {
            *x_ptr.add(i) = i as i32;
        }
        x.set_len(size);
    }
    assert_eq!(&*x, &[0, 1, 2, 3]);

    // 由于有别名保证，以下代码是合法的
    unsafe {
        let mut v = vec![0];
        let ptr1 = v.as_mut_ptr();
        ptr1.write(1);
        let ptr2 = v.as_mut_ptr();
        ptr2.write(2);
        // 值得注意的是，对 `ptr2` 的写入并没有 ** 使 `ptr1` 无效
        ptr1.write(3);
        println!("{:?}", v);
    }

    // allocator() 返回底层分配器的引用  夜间版方法
    let vec = vec![1, 2, 3, 4];
    let _vec_alloc = vec.allocator();

    // unsafe fn set_len()  强制将Vec长度改为 new_len。
    // 这是一种低级操作，不维护类型的正常不变性。通常情况下，改变向量的长度应使用安全操作 如 truncate, resize, extend, or clear.
    // new_len 必须小于或等于 capacity()。
    // 位于 old_len.new_len 处的元素必须初始化。
    // 这种方法适用于将向量作为其他代码的缓冲区的情况

    // 虽然下面的示例很合理，但由于在调用 set_len 之前没有释放内部向量，因此存在内存泄漏：
    let mut vec = vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]];

    // vec.clear();
    unsafe {
        vec.set_len(0);
    }
    // 通常情况下，在这里可以使用 clear 来正确删除内容，从而避免内存泄漏。

    // swap_remove()  从Vec中删除一个元素并返回。
    // 被移除的元素会被Vec的最后一个元素替换。
    // 这样做不会保留剩余元素的排序，但也是 O(1)。如果需要保留元素顺序，请使用 remove
    let mut v = vec!["foo", "bar", "baz", "qux"];
    assert_eq!(v.swap_remove(1), "bar");
    assert_eq!(v, ["foo", "qux", "baz"]);

    assert_eq!(v.swap_remove(0), "foo");
    assert_eq!(v, ["baz", "qux"]);

    // insert() 在指定的索引位置插入一个元素，并将其后的所有元素向后移动。
    // 如果 index > len，则会慌乱。
    let mut vec = vec![1, 2, 3, 5];
    vec.insert(3, 4);
    assert_eq!(vec, [1, 2, 3, 4, 5]);
    // 时间复杂性
    // 耗时 O(Vec::len)。插入索引之后的所有项目都必须向右移动。在最坏的情况下，当插入索引为 0 时，所有元素都会被移位。

    // remove() 删除并返回Vec中指定索引位置的元素，并将其后的所有元素向前移动
    // 注意：由于会对剩余元素进行移位，因此它的最差性能为 O(n)。
    // 如果不需要保留元素的顺序，可以使用 swap_remove 代替。
    // 如果想从 Vec 的开头移除元素，请考虑使用 VecDeque::pop_front 代替。

    // 如果索引超出范围，就会慌乱
    let mut v = vec![1, 2, 3];
    assert_eq!(v.remove(1), 2);
    assert_eq!(v, [1, 3]);

    // retain() 只保留 指定的元素
    // 换句话说，删除 f(&e) 返回 false 的所有元素 e。
    // 该方法是就地操作，按原始顺序对每个元素访问一次，并保留所保留元素的顺序。
    let mut vec = vec![1, 2, 3, 4];
    vec.retain(|x| x % 2 == 0);
    assert_eq!(vec, [2, 4]);

    // 由于元素在原始顺序中被访问过一次，因此可以利用外部状态来决定保留哪些元素。
    let mut vec = vec![1, 2, 3, 4, 5];
    let keep = [false, true, true, false, true];
    let mut iter = keep.iter();
    vec.retain(|_| *iter.next().unwrap());
    assert_eq!(vec, [2, 3, 5]);

    // retain_mut() 只保留指定的元素，并传递一个可变引用
    // 换句话说，删除所有元素 e，使得 f(&mut e) 返回 false。
    // 这种方法是就地操作，按原始顺序对每个元素访问一次，并保留所保留元素的顺序。
    let mut vec = vec![1, 2, 3, 4];
    vec.retain_mut(|x| {
        if *x <= 3 {
            *x += 1;
            true
        } else {
            false
        }
    });

    assert_eq!(vec, [2, 3, 4]);

    // dedup_by_key() 会根据提供的键函数 key 移除Vec中连续的、键相同的元素，只保留第一个
    // 如果向量已排序，则会删除所有重复的元素。
    let mut vec = vec![10, 20, 21, 30, 20];
    vec.dedup_by_key(|i| *i / 10);
    println!("{:?}", vec);
    assert_eq!(vec, [10, 20, 30, 20]);
    /*
        使用闭包 |i| *i / 10 作为键函数，作用是将每个元素除以 10 并取整。这个闭包将向量中的每个元素除以 10 后的结果作为键。
        对于元素 10，键为 10 / 10 = 1
        对于元素 20，键为 20 / 10 = 2
        对于元素 21，键为 21 / 10 = 2
        对于元素 30，键为 30 / 10 = 3
        对于元素 20，键为 20 / 10 = 2
        由于 21 和 20 具有相同的键 2，21 会被移除。同理，如果有更多连续的相同键，它们也会被移除，只保留第一个。
    */

    // dedup_by() 移除Vec中满足给定相等关系的连续元素中除第一个元素以外的所有元素。
    // 闭包函数从Vec中传递两个元素的引用，并且必须确定这两个元素是否相等。
    // 元素传递的顺序与它们在切片中的顺序相反，因此如果 same_bucket(a,b)返回 true，则删除 a。
    // 如果向量已排序，则删除所有重复的向量。

    let mut vec = vec!["foo", "bar", "Bar", "baz", "bar"];
    vec.dedup_by(|a, b| a.eq_ignore_ascii_case(b));
    assert_eq!(vec, ["foo", "bar", "baz", "bar"]);

    // push() 将一个元素添加到集合的后面。
    let mut vec = vec![1, 2];
    vec.push(3);
    vec.push(4);
    assert_eq!(vec, [1, 2, 3, 4]);
    // 时间复杂性
    // 耗时 O(1)。如果推送后向量的长度超过其容量，则需要 O(capacity) 时间将向量的元素复制到一个更大的分配区。
    // 这一昂贵的操作被其允许的 O(1) 个插入容量所抵消

    // push_within_capacity()  如果有足够的空余容量，则追加一个元素，否则将与该元素一起返回错误信息。
    // 与 push 不同，该方法不会在容量不足时重新分配。调用者应使用 reserve 或 try_reserve 来确保有足够的容量。

    let vec = [1, 2, 3, 4, 5];
    let _ = from_iter_fallible(vec.iter());
    println!("{:?}", vec);

    // pop()  从Vec中移除最后一个元素并返回它，如果是空的，则返回 None。
    // 如果你想移除第一个元素，请考虑使用 VecDeque::pop_front 代替。
    let mut vec = vec![1, 2, 3];
    assert_eq!(vec.pop(), Some(3));
    assert_eq!(vec, [1, 2]);

    // pop_if() 如果闭包返回 true，则删除并返回Vec中的最后一个元素；如果返回 false 或Vec为空，则删除并返回 None。
    let mut vec = vec![1, 2, 3, 4];
    let pred = |x: &mut i32| *x % 2 == 0;

    assert_eq!(vec.pop_if(pred), Some(4));
    assert_eq!(vec, [1, 2, 3]);
    assert_eq!(vec.pop_if(pred), None);

    // append() 将 另一个Vec的所有元素移入到自己，并清空被移动的集合
    // 如果新容量超过 isize::MAX 字节，就会出现 Panics。
    let mut vec = vec![1, 2, 3];
    let mut vec2 = vec![4, 5, 6];
    vec.append(&mut vec2);
    assert_eq!(vec, [1, 2, 3, 4, 5, 6]);
    assert_eq!(vec2, []);

    // drain() 从Vec中批量移除指定范围的元素，并以迭代器的形式返回所有被移除的元素。
    // 如果迭代器在完全耗尽前被删除，则会删除剩余的删除元素。 返回的迭代器保留了Vec的可变借用，以优化其执行
    // 如果返回的迭代器离开作用域而没有被丢弃（例如由于 mem::forget 的原因），Vec可能会任意丢失和泄漏元素，包括超出范围的元素。
    let mut v = vec![1, 2, 3];
    let u: Vec<_> = v.drain(1..).collect();
    assert_eq!(v, &[1]);
    assert_eq!(u, &[2, 3]);

    // clear() 清除Vec，删除所有值。 注意，此方法对分配的Vec容量没有影响。
    let mut v = vec![1, 2, 3];
    v.clear();
    assert!(v.is_empty());

    // len() 返回Vec中元素的个数，也称为 长度
    let a = vec![1, 2, 3];
    assert_eq!(a.len(), 3);

    // is_empty() 如果Vec不包含任何元素，则返回 true。
    let mut v = Vec::new();
    assert!(v.is_empty());

    v.push(1);
    assert!(!vec.is_empty());

    // split_off(at: usize) 在给定索引处将集合一分为二
    // 返回一个新分配的Vec，其中包含 [at, len) 范围内的元素。 调用后，原来的向量将只包含 [0, at]，其容量保持不变。
    /*
     如果你想拥有整个Vec的内容和容量，请参阅 mem::take 或 mem::replace
     如果完全不需要返回的Vec，请参阅 Vec::truncate
     如果您想获得任意子片段的所有权，或者不一定要将删除的项目存储在Vec中，请参阅 Vec::drain。
    */
    let mut vec = vec![1, 2, 3];
    let vec2 = vec.split_off(1);
    assert_eq!(vec, [1]);
    assert_eq!(vec2, [2, 3]);

    // resize_with() 就地调整 Vec 的大小，使 len 等于 new_len
    // 如果 new_len 大于 len，Vec 将根据差值进行扩展，每个额外的槽都将由调用闭包 f 的结果填充
    // 如果 new_len 小于 len，则 Vec 会被截断
    // 此方法使用闭包在每次推送时创建新值
    // 如果你想克隆一个给定值，可以使用 Vec::resize
    // 如果想使用 Default 特质生成值，可以将 Default::default 作为第二个参数传递
    let mut vec = vec![1, 2, 3];
    vec.resize_with(5, Default::default);
    assert_eq!(vec, [1, 2, 3, 0, 0]);

    let mut vec = vec![];
    let mut p = 1;
    vec.resize_with(4, || {
        p *= 2;
        p
    });
    assert_eq!(vec, [2, 4, 8, 16]);

    // leak() 消耗并泄漏 Vec，返回内容的可变引用，&'a mut [T]。
    /*
        请注意，T 类型的生命周期必须长于所选的生命周期 "a"。 如果类型只有静态引用，或根本没有静态引用，则可以选择 "静态"。
        从 Rust 1.57 开始，此方法不会重新分配或缩小 Vec，因此泄漏的分配可能包括未使用的容量，而这些容量不属于返回的片段。
        该函数主要适用于在程序剩余生命周期内仍有效的数据。 丢弃返回的引用会导致内存泄漏。
    */

    let x = vec![1, 2, 3];
    let static_ref: &'static mut [usize] = x.leak();
    static_ref[0] += 1;
    assert_eq!(static_ref, &[2, 2, 3]);

    // spare_capacity_mut() 以 MaybeUninit<T> 的片段形式返回向量的剩余容量。
    // 在使用 set_len 方法将数据标记为初始化之前，可以使用返回的片段将数据填充到向量中（例如从文件中读取数据）。
    let mut v = Vec::with_capacity(10);

    let uninit = v.spare_capacity_mut();
    uninit[0].write(0);
    uninit[1].write(1);
    uninit[2].write(2);

    unsafe {
        v.set_len(3);
    }

    assert_eq!(&v, &[0, 1, 2]);

    // split_at_spare_mut() 以 T 的片段形式返回向量内容，以及以 MaybeUninit<T> 的片段形式返回向量的剩余容量。
    let mut v = vec![1, 1, 2];

    // 为 10 个元素预留足够大的额外空间。
    v.reserve(10);

    let (init, uninit) = v.split_at_spare_mut();
    let sum = init.iter().copied().sum::<u32>();

    uninit[0].write(sum);
    uninit[1].write(sum * 2);
    uninit[2].write(sum * 3);
    uninit[3].write(sum * 4);

    unsafe {
        let len = v.len();
        v.set_len(len + 4);
    }

    assert_eq!(&v, &[1, 1, 2, 4, 8, 12, 16]);

    // resize()  就地调整 Vec 的大小，使 len 等于 new_len
    // 如果 new_len 大于 len，Vec 将根据差值进行扩展，每个额外的槽都将填入值。 如果 new_len 小于 len，则 Vec 会被截断
    // 此方法要求 T 实现 Clone，以便克隆传递的值。
    // 如果您需要更大的灵活性 或想使用 Default 代替 Clone 请使用 Vec::resize_with
    // 如果只需要调整到较小的大小，请使用 Vec::truncate。
    let mut vec = vec!["hello"];
    vec.resize(3, "world");
    assert_eq!(vec, ["hello", "world", "world"]);

    let mut vec = vec![1, 2, 3, 4];
    vec.resize(2, 0);
    assert_eq!(vec, [1, 2]);

    // extend_from_slice() 克隆并追加片段中的所有元素到 Vec
    // 遍历其他片段，克隆每个元素，然后将其追加到此 Vec。 其他片段按顺序遍历
    // 请注意，该函数与 extend 函数相同，只是它专门用于处理切片。 如果 Rust 实现了专门化，该函数很可能会被弃用（但仍然可用）
    let mut vec = vec![1];
    vec.extend_from_slice(&[2, 3, 4]);
    assert_eq!(vec, [1, 2, 3, 4]);

    // extend_from_within() 将 src 范围内的元素复制到Vec的末端。
    let mut vec = vec![0, 1, 2, 3, 4];

    vec.extend_from_within(2..);
    assert_eq!(vec, [0, 1, 2, 3, 4, 2, 3, 4]);

    vec.extend_from_within(..2);
    assert_eq!(vec, [0, 1, 2, 3, 4, 2, 3, 4, 0, 1]);

    vec.extend_from_within(4..8);
    assert_eq!(vec, [0, 1, 2, 3, 4, 2, 3, 4, 0, 1, 4, 2, 3, 4]);

    // into_flattened() 获取一个 Vec<[T; N]>，并将其扁平化为一个 Vec<T>
    // 如果Vec的长度会超出 usize，则会慌乱。
    let mut vec = vec![[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    assert_eq!(vec.pop(), Some([7, 8, 9]));

    let mut flattened = vec.into_flattened();
    assert_eq!(flattened.pop(), Some(6));

    // dedup() 根据 PartialEq 特性实现，删除向量中连续重复的元素。 如果向量已排序，则删除所有重复元素。
    let mut vec = vec![1, 2, 2, 3, 2];
    vec.dedup();
    assert_eq!(vec, [1, 2, 3, 2]);

    // splice() 创建一个拼接迭代器，用给定的 replace_with 迭代器替换向量中的指定范围，并生成移除的项
    let mut v = vec![1, 2, 3, 4];
    let new = [7, 8, 9];
    let u: Vec<_> = v.splice(1..3, new).collect();
    assert_eq!(v, &[1, 7, 8, 9, 4]);
    assert_eq!(u, &[2, 3]);

    // extract_if() 创建一个迭代器，该迭代器使用闭包来确定是否要删除某个元素
    // 如果闭包返回 true，那么该元素将被移除并产生。 如果闭包返回 false，元素将保留在向量中，不会被迭代器产生。
    let mut numbers = vec![1, 2, 3, 4, 5, 6, 8, 9, 11, 13, 14, 15];

    let evens = numbers.extract_if(|x| *x % 2 == 0).collect::<Vec<_>>();
    let odds = numbers;

    assert_eq!(evens, vec![2, 4, 6, 8, 14]);
    assert_eq!(odds, vec![1, 3, 5, 9, 11, 13, 15]);

    // Methods from Deref<Target = [T]>

    // first() 返回Vec的第一个元素的不可变引用，如果为空，则返回 None
    let v = [10, 40, 30];
    assert_eq!(Some(&10), v.first());

    let w: &[i32] = &[];
    assert_eq!(None, w.first());

    // first_mut()  返回Vec第一个元素的可变指针，如果为空，则返回 None。
    let x = &mut [0, 1, 2];
    if let Some(first) = x.first_mut() {
        *first = 5;
    }
    assert_eq!(x, &[5, 1, 2]);

    let y: &mut [i32] = &mut [];
    assert_eq!(None, y.first_mut());

    // split_first() 返回Vec的第一个元素和所有其他元素的不可变引用，如果为空，则返回 None。
    let x = &[0, 1, 2];
    if let Some((first, elements)) = x.split_first() {
        assert_eq!(first, &0);
        assert_eq!(elements, &[1, 2]);
    }

    // split_first_mut()  返回Vec的第一个元素和所有其他元素的可变引用，如果为空，则返回 None
    let x = &mut [0, 1, 2];
    if let Some((first, elements)) = x.split_first_mut() {
        *first = 3;
        elements[0] = 4;
        elements[1] = 5;
    }

    // split_last() 返回Vec的最后一个元素和所有其他元素的不可变引用，如果为空，则返回 None。
    let x = &[0, 1, 2];
    if let Some((last, elements)) = x.split_last() {
        assert_eq!(last, &2);
        assert_eq!(elements, &[0, 1]);
    }

    // split_last_mut()  返回Vec的最后一个元素和所有其他元素的可变引用，如果为空，则返回 None
    let x = &mut [0, 1, 2];
    if let Some((last, elements)) = x.split_last_mut() {
        *last = 5;
        elements[0] = 3;
        elements[1] = 4;
    }

    // last() 返回Vec的最后一个元素的不可变引用，如果为空，则返回 None。
    let v = [10, 40, 20];
    assert_eq!(Some(&20), v.last());

    let w: &[i32] = &[];
    assert_eq!(None, w.last());

    // last_mut() 返回切片中最后一个元素的可变引用，如果为空，则返回 None。
    let x = &mut [0, 1, 2];

    if let Some(last) = x.last_mut() {
        *last = 10;
    }
    assert_eq!(x, &[0, 1, 10]);

    let y: &mut [i32] = &mut [];
    assert_eq!(None, y.last_mut());

    // first_chunk() 返回切片中前 N 个项目的数组引用。 如果切片长度小于 N，则返回 None。
    let u = [10, 20, 30];
    assert_eq!(Some(&[10, 20]), u.first_chunk::<2>());

    let v: &[i32] = &[10];
    assert_eq!(None, v.first_chunk::<2>());

    let w: &[i32] = &[];
    assert_eq!(Some(&[]), w.first_chunk());

    // first_chunk_mut() 返回切片中前 N 个项的可变数组引用。 如果切片的长度小于 N，则返回 None。
    let x = &mut [0, 1, 2];

    if let Some(first) = x.first_chunk_mut::<2>() {
        first[0] = 5;
        first[1] = 4;
    }

    assert_eq!(x, &[5, 4, 2]);

    assert_eq!(None, x.first_chunk_mut::<4>());

    // split_first_chunk() 返回一个数组的不可变引用，指向切片中的前 N 个项目和剩余的切片。 如果切片的长度小于 N，则返回 None。
    let x = &[0, 1, 2];

    if let Some((first, elements)) = x.split_first_chunk::<2>() {
        assert_eq!(first, &[0, 1]);
        assert_eq!(elements, &[2]);
    }

    assert_eq!(None, x.split_first_chunk::<4>());

    // split_first_chunk_mut()  返回一个数组的可变引用，指向切片中的前 N 个项目和剩余的切片。 如果切片长度小于 N，则返回 None
    let x = &mut [0, 1, 2];

    if let Some((first, elements)) = x.split_first_chunk_mut::<2>() {
        first[0] = 3;
        first[1] = 4;
        elements[0] = 5;
    }
    assert_eq!(x, &[3, 4, 5]);

    assert_eq!(None, x.split_first_chunk_mut::<4>());

    // split_last_chunk() 返回片段中最后 N 个项目和剩余片段的 不可变引用。 如果片段的长度小于 N，则返回 None。
    let x = &[0, 1, 2];
    if let Some((elements, last)) = x.split_last_chunk::<2>() {
        assert_eq!(elements, &[0]);
        assert_eq!(last, &[1, 2]);
    }

    // split_last_chunk_mut() 返回一个数组的可变引用，指向片段中的最后 N 个项目和剩余的片段。 如果片段的长度小于 N，则返回 None。
    let x = &mut [0, 1, 2];
    if let Some((elements, last)) = x.split_last_chunk_mut::<2>() {
        elements[0] = 3;
        last[0] = 4;
        last[1] = 5;
    }
    assert_eq!(x, &[3, 4, 5]);

    assert_eq!(None, x.split_last_chunk_mut::<4>());

    // last_chunk()  返回切片中最后 N 个项目的数组引用。 如果切片长度小于 N，则返回 None。
    let x = &[0, 1, 2];
    assert_eq!(Some(&[1, 2]), x.last_chunk::<2>());

    let v: &[i32] = &[10];
    assert_eq!(None, v.last_chunk::<2>());

    let w: &[i32] = &[];
    assert_eq!(Some(&[]), w.last_chunk::<0>());

    // last_chunk_mut() 返回切片中最后 N 个项目 数组的可变引用。 如果切片长度小于 N，则返回 None。
    let x = &mut [0, 1, 2];

    if let Some(last) = x.last_chunk_mut::<2>() {
        last[0] = 10;
        last[1] = 20;
    }

    assert_eq!(x, &[0, 10, 20]);
    assert_eq!(None, x.last_chunk_mut::<4>());

    // get() 根据索引类型，返回元素或子片段的不可变引用
    /*
        如果给定位置，则返回该位置元素的引用；如果超出范围，则返回 None。
        如果给定范围，则返回该范围对应的子片段；如果超出范围，则返回 None。
    */
    let v = [10, 30, 50];
    assert_eq!(Some(&50), v.get(2));
    println!("{:?}", v.get(0..2));
    assert_eq!(Some(&[10, 30][..]), v.get(0..2));
    assert_eq!(None, v.get(3));
    assert_eq!(None, v.get(0..4));

    // get_mut() 根据索引类型（参见 get），返回元素或子片段的可变引用；如果索引越界，则返回 None
    let x = &mut [0, 1, 2];

    if let Some(elem) = x.get_mut(1) {
        *elem = 42;
    }
    assert_eq!(x, &[0, 42, 2]);

    //  get_unchecked() 返回元素或子片段的不可变引用，不进行边界校验
    let x = &[1, 2, 4];
    unsafe {
        assert_eq!(x.get_unchecked(1), &2);
    }

    // get_unchecked_mut() 返回元素或子片段的可变引用，不进行边界校验
    let x = &mut [1, 2, 4];

    unsafe {
        let elem = x.get_unchecked_mut(1);
        *elem = 13;
    }

    assert_eq!(x, &[1, 13, 4]);

    // as_ptr()  返回切片缓冲区的原始指针
    // 调用者必须确保切片的生命周期超过此函数返回的指针，否则它最终将指向垃圾。
    // 修改该片引用的容器可能会导致其缓冲区被重新分配，这也会使指向它的任何指针失效。
    let x = &[1, 2, 4];
    let x_ptr = x.as_ptr();

    unsafe {
        for i in 0..x.len() {
            assert_eq!(x.get_unchecked(i), &*x_ptr.add(i));
        }
    }

    // as_mut_ptr() 返回指向切片缓冲区的不安全可变指针
    // 调用者必须确保切片的生命周期超过此函数返回的指针，否则它最终将指向垃圾
    // 修改该片引用的容器可能会导致其缓冲区被重新分配，这也会使指向它的任何指针失效。
    let x = &mut [1, 2, 4];
    let x_ptr = x.as_mut_ptr();

    unsafe {
        for i in 0..x.len() {
            *x_ptr.add(i) += 2;
        }
    }
    assert_eq!(x, &[3, 4, 6]);

    // as_ptr_range() 返回横跨切片的两个原始指针
    let a = [1, 2, 3];
    let x = &a[1] as *const _;
    let y = &5 as *const _;

    assert!(a.as_ptr_range().contains(&x));
    assert!(!a.as_ptr_range().contains(&y));

    // as_mut_ptr_range() 返回横跨切片的两个不安全可变指针

    // swap() 交换切片中的两个元素 如果 a 等于 b，则保证元素的值不会改变
    let mut v = ["a", "b", "c", "d", "e"];
    v.swap(2, 4);
    assert!(v == ["a", "b", "e", "d", "c"]);

    // swap_unchecked() 交换片段中的两个元素，不进行边界检查。
    let mut v = ["a", "b", "c", "d", "e"];
    unsafe {
        v.swap_unchecked(2, 4);
    }
    assert!(v == ["a", "b", "e", "d", "c"]);

    // reverse() 将切片中元素的顺序颠倒
    let mut v = [1, 2, 3];
    v.reverse();
    assert!(v == [3, 2, 1]);

    // iter() 返回切片上的迭代器。 该迭代器产生从开始到结束的所有项目
    let x = &[1, 2, 4];
    let mut iterator = x.iter();

    assert_eq!(iterator.next(), Some(&1));
    assert_eq!(iterator.next(), Some(&2));
    assert_eq!(iterator.next(), Some(&4));
    assert_eq!(iterator.next(), None);

    // iter_mut() 返回一个可修改每个值的迭代器。 该迭代器产生从开始到结束的所有项目
    let x = &mut [1, 2, 4];
    for elem in x.iter_mut() {
        *elem += 2;
    }
    assert_eq!(x, &[3, 4, 6]);

    // windows() 返回长度为 size 的所有连续窗口的迭代器。 窗口会重叠。 如果切片长度不够，迭代器将不返回任何值。
    let slice = ['l', 'o', 'r', 'e', 'm'];
    let mut iter = slice.windows(3);
    assert_eq!(iter.next().unwrap(), &['l', 'o', 'r']);
    assert_eq!(iter.next().unwrap(), &['o', 'r', 'e']);
    assert_eq!(iter.next().unwrap(), &['r', 'e', 'm']);
    assert!(iter.next().is_none());

    let slice = ['l', 'o', 'r', 'e', 'm'];
    let mut iter = slice.windows(4);
    assert_eq!(iter.next().unwrap(), &['l', 'o', 'r', 'e']);
    assert_eq!(iter.next().unwrap(), &['o', 'r', 'e', 'm']);
    assert!(iter.next().is_none());

    let slice = ['f', 'o', 'o'];
    let mut iter = slice.windows(4);
    assert!(iter.next().is_none());

    // 没有 windows_mut，因为它的存在会让安全代码违反 "一次只能对同一事物使用一个 &mut "的规则。
    // 不过，有时可以将 Cell::as_slice_of_cells 与 windows 结合使用，以实现类似的功能：
    // !!! 有难度  画画图
    let mut array = ['R', 'u', 's', 't', ' ', '2', '0', '1', '5'];
    let slice = &mut array[..];
    let slice_of_cells: &[Cell<char>] = Cell::from_mut(slice).as_slice_of_cells();
    for w in slice_of_cells.windows(3) {
        Cell::swap(&w[0], &w[2])
    }
    assert_eq!(array, ['s', 't', ' ', '2', '0', '1', '5', 'u', 'R']);

    // chunks() 每次从片段的开头开始，返回片段中 chunk_size 元素的迭代器。
    // 分块是切片，不会重叠。 如果 chunk_size 没有除以切片的长度，那么最后一个分块的长度就不是 chunk_size。
    let slice = ['l', 'o', 'r', 'e', 'm'];
    let mut iter = slice.chunks(2);
    assert_eq!(iter.next().unwrap(), &['l', 'o']);
    assert_eq!(iter.next().unwrap(), &['r', 'e']);
    assert_eq!(iter.next().unwrap(), &['m']);
    assert!(iter.next().is_none());

    // chunks_mut() 每次从片段的开头开始，返回片段中 chunk_size 元素的迭代器
    // 分块是可变的片段，不会重叠。 如果 chunk_size 没有除以切片的长度，那么最后一个分块的长度就不是 chunk_size
    let v = &mut [0, 0, 0, 0, 0];
    let mut count = 1;
    for chunk in v.chunks_mut(2) {
        for elem in chunk.iter_mut() {
            *elem += count;
        }
        count += 1;
    }
    assert_eq!(v, &[1, 1, 2, 2, 3]);

    // chunks_exact()  每次从片段的开头开始，返回片段中 chunk_size 元素的迭代器
    // 数据块是切片，不会重叠。 如果 chunk_size 不等于切片的长度，那么最后到 chunk_size-1 的元素将被省略
    // 可以通过迭代器的 remainder 函数获取。
    let slice = ['l', 'o', 'r', 'e', 'm'];
    let mut iter = slice.chunks_exact(2);
    assert_eq!(iter.next().unwrap(), &['l', 'o']);
    assert_eq!(iter.next().unwrap(), &['r', 'e']);
    assert!(iter.next().is_none());
    assert_eq!(iter.remainder(), &['m']);

    // chunks_exact_mut() 每次从片段的开头开始，返回片段中 chunk_size 元素的迭代器
    // 分块是可变的片段，不会重叠。 如果 chunk_size 不等于切片的长度，那么最后到 chunk_size-1 的元素将被省略，
    // 可以通过迭代器的 into_remainder 函数获取
    let v = &mut [0, 0, 0, 0, 0];
    let mut count = 1;

    for chunk in v.chunks_exact_mut(2) {
        for elem in chunk.iter_mut() {
            *elem += count;
        }
        count += 1;
    }

    assert_eq!(v, &[1, 1, 2, 2, 0]);

    // as_chunks_unchecked()  假定没有余数，将切片分割成 N 元素数组的切片
    // 切片正好分割成 N 个元素块（又称 self.len() % N == 0）。 N != 0。
    let slice: &[char] = &['l', 'o', 'r', 'e', 'm', '!'];

    let chunks: &[[char; 1]] =
        // SAFETY: 1-element chunks never have remainder
        unsafe { slice.as_chunks_unchecked() };
    assert_eq!(chunks, &[['l'], ['o'], ['r'], ['e'], ['m'], ['!']]);
    let chunks: &[[char; 3]] =
        // SAFETY: The slice length (6) is a multiple of 3
        unsafe { slice.as_chunks_unchecked() };
    assert_eq!(chunks, &[['l', 'o', 'r'], ['e', 'm', '!']]);

    // These would be unsound:
    // let chunks: &[[_; 5]] = slice.as_chunks_unchecked() // The slice length is not a multiple of 5
    // let chunks: &[[_; 0]] = slice.as_chunks_unchecked() // Zero-length chunks are never allowed

    // as_chunks() 从片段的起始位置开始，将片段分割成由 N 个元素组成的片段，以及长度严格小于 N 的剩余片段
    let slice: &[char] = &['l', 'o', 'r', 'e', 'm'];
    let (chunks, remainder) = slice.as_chunks();
    assert_eq!(chunks, &[['l', 'o', 'r']]);
    assert_eq!(remainder, &['e', 'm']);

    let slice = ['R', 'u', 's', 't'];
    let (chunks, []) = slice.as_chunks::<2>() else {
        panic!("slice disn't have even length")
    };
    assert_eq!(chunks, &[['R', 'u'], ['s', 't']]);

    // rchunks() 每次从片段末尾开始，返回片段中 chunk_size 元素的迭代器
    // 分块是切片，不会重叠。 如果 chunk_size 没有除以切片的长度，那么最后一个分块的长度就不是 chunk_size。
    let slice: &[char] = &['l', 'o', 'r', 'e', 'm'];
    let mut iter = slice.rchunks(2);
    assert_eq!(iter.next().unwrap(), &['e', 'm']);
    assert_eq!(iter.next().unwrap(), &['o', 'r']);
    assert_eq!(iter.next().unwrap(), &['l']);
    assert!(iter.next().is_none());

    // rchunks_mut() 每次从片段末尾开始，返回片段中 chunk_size 元素的迭代器
    // 分块是可变的片段，不会重叠。 如果 chunk_size 没有除以切片的长度，那么最后一个分块的长度就不是 chunk_size
    let v = &mut [0, 0, 0, 0, 0];
    let mut count = 1;

    for chunk in v.rchunks_mut(2) {
        for elem in chunk.iter_mut() {
            *elem += count;
        }
        count += 1;
    }
    assert_eq!(v, &[3, 2, 2, 1, 1]);

    // rchunks_exact() 每次从片段末尾开始，返回片段中 chunk_size 元素的迭代器
    // 数据块是切片，不会重叠。 如果 chunk_size 不等于切片的长度，那么最后到 chunk_size-1 的元素将被省略，
    // 可以通过迭代器的 remainder 函数获取
    let slice: &[char] = &['l', 'o', 'r', 'e', 'm'];
    let mut iter = slice.rchunks_exact(2);
    assert_eq!(iter.next().unwrap(), &['e', 'm']);
    assert_eq!(iter.next().unwrap(), &['o', 'r']);
    assert!(iter.next().is_none());
    assert_eq!(iter.remainder(), &['l']);

    // rchunks_exact_mut()  每次从片段末尾开始，返回片段中 chunk_size 元素的迭代器
    // 分块是可变的片段，不会重叠。 如果 chunk_size 不等于切片的长度，那么最后到 chunk_size-1 的元素将被省略，
    // 可以通过迭代器的 into_remainder 函数获取。
    let v = &mut [0, 0, 0, 0, 0];
    let mut count = 1;

    for chunk in v.rchunks_exact_mut(2) {
        for elem in chunk.iter_mut() {
            *elem += count;
        }
        count += 1;
    }
    assert_eq!(v, &[0, 2, 2, 1, 1]);

    // chunk_by() 返回一个迭代器，该迭代器会使用闭包来分隔产生非重叠元素的片段。
    // 每一对连续元素都会调用闭包，也就是说，它会在 slice[0] 和 slice[1] 上调用，然后是 slice[1] 和 slice[2]，依此类推。
    let slice = &[1, 1, 1, 3, 3, 2, 2, 2];
    let mut iter = slice.chunk_by(|a, b| a == b);

    assert_eq!(iter.next(), Some(&[1, 1, 1][..]));
    assert_eq!(iter.next(), Some(&[3, 3][..]));
    assert_eq!(iter.next(), Some(&[2, 2, 2][..]));
    assert_eq!(iter.next(), None);
    // 这种方法可用于提取已排序的子切片
    let slice = &[1, 1, 2, 3, 2, 3, 2, 3, 4];
    let mut iter = slice.chunk_by(|a, b| a <= b);
    assert_eq!(iter.next(), Some(&[1, 1, 2, 3][..]));
    assert_eq!(iter.next(), Some(&[2, 3][..]));
    assert_eq!(iter.next(), Some(&[2, 3, 4][..]));
    assert_eq!(iter.next(), None);

    // chunk_by_mut() 返回分片上的迭代器，该迭代器使用闭包来分隔元素的非重叠可变运行

    let slice = &mut [1, 1, 1, 3, 3, 2, 2, 2];

    let mut iter = slice.chunk_by_mut(|a, b| a == b);

    assert_eq!(iter.next(), Some(&mut [1, 1, 1][..]));
    assert_eq!(iter.next(), Some(&mut [3, 3][..]));
    assert_eq!(iter.next(), Some(&mut [2, 2, 2][..]));
    assert_eq!(iter.next(), None);

    // split_at() 按索引将一个切片分成两个。
    let v = [1, 2, 3, 4, 5, 6];
    {
        let (left, right) = v.split_at(0);
        assert_eq!(left, []);
        assert_eq!(right, [1, 2, 3, 4, 5, 6]);
    }

    {
        let (left, right) = v.split_at(2);
        assert_eq!(left, [1, 2]);
        assert_eq!(right, [3, 4, 5, 6]);
    }
    {
        let (left, right) = v.split_at(6);
        assert_eq!(left, [1, 2, 3, 4, 5, 6]);
        assert_eq!(right, []);
    }

    // split_at_mut() 在索引处将一个可变片段分为两个
    let mut v = [1, 0, 3, 0, 5, 6];
    let (left, right) = v.split_at_mut(2);
    assert_eq!(left, [1, 0]);
    assert_eq!(right, [3, 0, 5, 6]);
    left[1] = 2;
    right[1] = 4;
    assert_eq!(v, [1, 2, 3, 4, 5, 6]);

    // split_at_unchecked() 在一个索引处将一个切片一分为二，但不进行边界校验
    let v = [1, 2, 3, 4, 5, 6];
    unsafe {
        let (left, right) = v.split_at_unchecked(0);
        assert_eq!(left, []);
        assert_eq!(right, [1, 2, 3, 4, 5, 6]);
    }

    unsafe {
        let (left, right) = v.split_at_unchecked(2);
        assert_eq!(left, [1, 2]);
        assert_eq!(right, [3, 4, 5, 6]);
    }

    unsafe {
        let (left, right) = v.split_at_unchecked(6);
        assert_eq!(left, [1, 2, 3, 4, 5, 6]);
        assert_eq!(right, []);
    }

    // split_at_mut_unchecked() 在索引处将一个可变片段一分为二，但不进行边界校验
    let mut v = [1, 0, 3, 0, 5, 6];
    unsafe {
        let (left, right) = v.split_at_mut_unchecked(2);
        assert_eq!(left, [1, 0]);
        assert_eq!(right, [3, 0, 5, 6]);
        left[1] = 2;
        right[1] = 4;
    }
    assert_eq!(v, [1, 2, 3, 4, 5, 6]);

    // split() 根据符合闭包条件的元素进行分割
    let slice = [10, 40, 33, 20, 30, 10];
    let mut iter = slice.split(|num| num % 3 == 0);

    assert_eq!(iter.next().unwrap(), &[10, 40]);
    assert_eq!(iter.next().unwrap(), &[20]);
    assert_eq!(iter.next().unwrap(), &[10]);
    assert!(iter.next().is_none());

    // 如果第一个元素被匹配，迭代器返回的第一个项目将是空片段
    // 同样，如果片段中的最后一个元素被匹配，迭代器返回的最后一个项目将是空片段
    let slice = [10, 40, 33];
    let mut iter = slice.split(|num| num % 3 == 0);
    assert_eq!(iter.next().unwrap(), &[10, 40]);
    assert_eq!(iter.next().unwrap(), &[]);
    assert!(iter.next().is_none());

    // 如果两个匹配元素直接相邻，则它们之间会出现一个空切片
    let slice = [10, 6, 33, 20];
    let mut iter = slice.split(|num| num % 3 == 0);

    assert_eq!(iter.next().unwrap(), &[10]);
    assert_eq!(iter.next().unwrap(), &[]);
    assert_eq!(iter.next().unwrap(), &[20]);
    assert!(iter.next().is_none());

    // split_mut() 根据符合闭包条件的元素进行分割 返回可变的片段
    let mut v = [10, 40, 30, 20, 60, 50];
    for group in v.split_mut(|num| *num % 3 == 0) {
        group[0] = 1;
    }
    assert_eq!(v, [1, 40, 30, 1, 60, 1]);

    // split_inclusive() 根据符合闭包条件的元素进行分割  符合条件的元素会被保留下来
    let slice = [10, 40, 33, 20];
    let mut iter = slice.split_inclusive(|num| num % 3 == 0);
    assert_eq!(iter.next().unwrap(), &[10, 40, 33]);
    assert_eq!(iter.next().unwrap(), &[20]);
    assert!(iter.next().is_none());

    // 如果片段的最后一个元素被匹配，则该元素将被视为前一个片段的终止符。 该分片将是迭代器返回的最后一个项目
    let slice = [3, 10, 40, 33];
    let mut iter = slice.split_inclusive(|num| num % 3 == 0);
    assert_eq!(iter.next().unwrap(), &[3]);
    assert_eq!(iter.next().unwrap(), &[10, 40, 33]);
    assert!(iter.next().is_none());

    // split_inclusive_mut() 返回由匹配 pred 的元素分隔的可变子片段的迭代器。 匹配的元素作为终止符包含在前一个子片段中
    let mut v = [10, 40, 30, 20, 60, 50];
    for group in v.split_inclusive_mut(|num| *num % 3 == 0) {
        let terminator_idx = group.len() - 1;
        group[terminator_idx] = 1;
    }
    assert_eq!(v, [10, 40, 1, 20, 1, 1]);

    // rsplit() 返回由匹配 pred 的元素分隔的子片段的迭代器，从片段末尾开始向后返回。 子片中不包含匹配的元素
    let slice = [11, 22, 33, 0, 44, 55];
    let mut iter = slice.rsplit(|num| *num == 0);

    assert_eq!(iter.next().unwrap(), &[44, 55]);
    assert_eq!(iter.next().unwrap(), &[11, 22, 33]);
    assert_eq!(iter.next(), None);

    // 与 split() 一样，如果第一个或最后一个元素被匹配，迭代器返回的第一个（或最后一个）项将是空片段
    let v = &[0, 1, 1, 2, 3, 5, 8];
    let mut it = v.rsplit(|n| *n % 2 == 0);

    assert_eq!(it.next().unwrap(), &[]);
    assert_eq!(it.next().unwrap(), &[3, 5]);
    assert_eq!(it.next().unwrap(), &[1, 1]);
    assert_eq!(it.next().unwrap(), &[]);
    assert_eq!(it.next(), None);

    // rsplit_mut() 返回由匹配 pred 的元素分隔的可变子片段的迭代器，从片段末尾开始向后返回。 子片中不包含匹配的元素。
    let mut v = [100, 400, 300, 200, 600, 500];

    let mut count = 0;
    for group in v.rsplit_mut(|num| *num % 3 == 0) {
        count += 1;
        group[0] = count;
    }
    assert_eq!(v, [3, 400, 300, 2, 600, 1]);

    // splitn() 返回由匹配 pred 的元素分隔的子片段的迭代器，最多只能返回 n 个项目。 匹配的元素不包含在子片段中
    // 根据参数来决定返回几个片段
    // 返回的最后一个元素（如果有的话）将包含切片的剩余部分
    let v = [10, 40, 30, 20, 60, 50];

    for group in v.splitn(2, |num| *num % 3 == 0) {
        println!("{:?}", group);
    }

    // splitn_mut() 返回由匹配 pred 的元素分隔的可变子片段的迭代器，最多只能返回 n 个项目。 匹配的元素不包含在子片段中
    // 返回的最后一个元素（如果有的话）将包含切片的剩余部分
    let mut v = [10, 40, 30, 20, 60, 50];
    for group in v.splitn_mut(2, |num| *num % 3 == 0) {
        group[0] = 1;
    }
    assert_eq!(v, [1, 40, 30, 1, 60, 50]);

    // rsplitn() 返回子片段的迭代器，该迭代器由匹配 pred 的元素分隔，最多只能返回 n 个项目。 
    // 该迭代器从分片的末尾开始向后迭代。 子片段中不包含匹配的元素
    let v = [10, 40, 30, 20, 60, 50];
    for group in v.rsplitn(2, |num| *num % 3 == 0) {
        println!("rsplitn={:?}", group);
    }

    // rsplitn_mut() 返回子片段的迭代器，该迭代器由匹配 pred 的元素分隔，最多只能返回 n 个项目。 
    // 该迭代器从分片的末尾开始向后迭代。 子片段中不包含匹配的元素。
    let mut s = [10, 40, 30, 20, 60, 50];
    for group in s.rsplitn_mut(2, |num| *num % 3 == 0) {
        group[0] = 1;
    }
    assert_eq!(s, [1, 40, 30, 20, 60, 1]);

    // contains() 如果片段中包含给定值的元素，则返回 true
    // 时间复杂度是O(n)
    // 请注意，如果您有一个已排序的片段，二进制搜索可能会更快

    let v = [10, 40, 30];
    assert!(v.contains(&10));
    assert!(!v.contains(&1001));

    // 如果没有 &T，但有可以与之比较的其他值（例如，String 实现 PartialEq<str>），则可以使用 iter().any
    let v = [String::from("hello"), String::from("world")];
    assert!(v.iter().any(|e| e == "hello"));
    assert!(!v.iter().any(|e| e == "hi"));


    // starts_with() 如果slice 是切片的前缀或等于切片，则返回 true。
    let v = [10, 40, 30];
    assert!(v.starts_with(&[10]));
    assert!(v.starts_with(&[10, 40]));
    assert!(v.starts_with(&v));
    assert!(!v.starts_with(&[50]));
    assert!(!v.starts_with(&[10, 50]));

    // 如果slice是空切片，总是返回 true
    let v = &[10, 40, 30];
    assert!(v.start_with(&[]));
    let v: &[u8] = &[];
    assert!(v.starts_with(&[]));


}
fn from_iter_fallible<T>(iter: impl Iterator<Item=T>) -> Result<Vec<T>, TryReserveError> {
    let mut vec = Vec::new();
    for value in iter {
        if let Err(value) = vec.push_within_capacity(value) {
            vec.try_reserve(1)?;
            let _ = vec.push_within_capacity(value);
        }
    }
    Ok(vec)
}

fn process_data2(data: &[u32]) -> Result<Vec<u32>, TryReserveError> {
    let mut output = Vec::new();

    output.try_reserve_exact(data.len())?;

    output.extend(data.iter().map(|&val| val * 2 + 5));

    Ok(output)
}

fn process_data(data: &[u32]) -> Result<Vec<u32>, TryReserveError> {
    let mut output = Vec::new();

    // 预留内存，如果无法预留则退出
    output.try_reserve(data.len())?;

    // 现在我们知道，在我们复杂的工作中，它不会发生 OOM
    output.extend(data.iter().map(|&val| val * 2 + 5));
    Ok(output)
}

fn read_slice(slice: &[usize]) {
    println!("{:?}", slice);
}
