use std::fmt::{Debug, Display};

fn main() {
    // Trait：定义共同行为
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());
    // 调用默认实现方法
    println!("default impl {}", tweet.default_impl());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());
    // 调用默认实现方法
    println!("default impl {}", article.default_impl());

    println!("================================================");
    // 调用trait作为参数的方法
    notify(&tweet);
    notify(&article);

    let _s = 22.to_string();
}

// 使用 trait bound 有条件地实现方法
struct Pair<T> {
    x: T,
    y: T,
}

// 任何实例都可以调用new方法
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Pair { x, y }
    }
}

// 只有那些为 T 类型实现了 PartialOrd trait（来允许比较） 和 Display trait（来启用打印）的 Pair<T> 才会实现 cmp_display 方法
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x > self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// 尝试返回 NewsArticle 或 Tweet。这不能编译  后面会解决这个问题
/*
fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from(
                "Penguins win the Stanley Cup Championship!",
            ),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        }
    } else {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            retweet: false,
        }
    }
}
*/

// 返回实现了 trait 的类型
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

// 通过 where 简化 trait bound
fn _some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    55
}
fn _some_function_siple<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    55
}

// 通过 + 指定多个 trait bound
// 如果 notify 需要显示 item 的格式化形式，同时也要使用 summarize 方法
// 那么 item 就需要同时实现两个不同的 trait：Display 和 Summary。这可以通过 + 语法实现：
fn _notify_more(summary: &(impl Summary + Display)) {}
// + 语法也适用于泛型的 trait bound
fn _notify_more_bound<T: Summary + Display>(summary: &T) {}

// Trait Bound 语法
// impl Trait 语法适用于直观的例子，它实际上是一种较长形式我们称为 trait bound语法的语法糖。它看起来像
fn _notify_original<T: Summary>(summary: &T) {
    println!("Breaking news! {}", summary.summarize());
    println!("default {}", summary.default_impl());
}

// impl Trait 很方便，适用于短小的例子。更长的 trait bound 则适用于更复杂的场景
// 例如，可以获取两个实现了 Summary 的参数。使用 impl Trait 的语法看起来像这样：
fn _notify_second(summary1: &impl Summary, summary2: &impl Summary) {}

// 这适用于 summary1 和 summary2 允许是不同类型的情况（只要它们都实现了 Summary）
// 不过如果你希望强制它们都是相同类型呢？这只有在使用 trait bound 时才有可能
fn _notify_same<T: Summary>(summary1: &T, summary2: &T) {}
// 泛型 T 被指定为 summary1 和 summary2的参数限制，如此传递给参数 summary1 和 summary2 值的具体类型必须一致。

// trait 作为参数
fn notify(summary: &impl Summary) {
    println!("Breaking news! {}", summary.summarize());
    println!("default {}", summary.default_impl());
}

// 可以将 Summary 引入作用域以便为其自己的类型实现该 trait。
// 需要注意的限制是，只有在 trait 或类型至少有一个属于当前 crate 时，我们才能对类型实现该 trait

// 自定义类型 Tweet 实现如标准库中的 Display trait，这是因为 Tweet 类型位于 crate 本地的作用域中

// 但是不能为外部类型实现外部 trait  例如 在本地 crate 中为 Vec<T> 实现 Display trait。

// 这个限制是被称为 相干性（coherence）的程序属性的一部分，或者更具体的说是 孤儿规则（orphan rule），其得名于不存在父类型

// 这条规则确保了其他人编写的代码不会破坏你代码，反之亦然。
// 没有这条规则的话，两个 crate 可以分别对相同类型实现相同的 trait，而 Rust 将无从得知应该使用哪一个实现

// 为类型实现 trait

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// trait 体中可以有多个方法：一行一个方法签名且都以分号结尾
trait Summary {
    fn summarize(&self) -> String;

    // 默认实现
    // 默认实现允许调用相同 trait 中的其他方法，哪怕这些方法没有默认实现
    fn default_impl(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }

    fn summarize_author(&self) -> String;
}
