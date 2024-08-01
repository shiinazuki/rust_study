use chapt19_advance_feature::HelloMacro;

fn main() {
    // 宏

    // 使用 macro_rules! 的 声明（Declarative）宏，和三种 过程（Procedural）宏
    /*
        自定义 #[derive] 宏在结构体和枚举上指定通过 derive 属性添加的代码
        类属性（Attribute-like）宏定义可用于任意项的自定义属性
        类函数宏看起来像函数不过作用于作为参数传递的 token
    */

    // 使用 macro_rules! 的声明宏用于通用元编程
    let v  = vec![1, 2, 3];
    println!("{:?}", v);


    // 用于从属性生成代码的过程宏
    // 有三种类型的过程宏（自定义派生（derive），类属性和类函数），不过它们的工作方式都类似

    Pancakes::hello_macro();

    // 类属性宏
    // derive 只能用于结构体和枚举；属性还可以用于其它的项，比如函数


    // 类函数宏
    // 类函数宏例子是可以像这样被调用的 sql! 宏
    // let sql = sql!(SELECT * FROM table_name WHERE id=1);

}

// #[proc_macro]
// pub fn sql(input: TokenStream) -> TokenStream {

// #[route(GET, "/")]
// fn index() {}



// 如何编写自定义 derive 宏
#[derive(HelloMacro)]
struct Pancakes;

// impl HelloMacro for Pancakes {
//     fn hello_macro() {
//         println!("Hello, Macro! My name is Pancakes!");
//     }
// }


trait HelloMacro {
    fn hello_macro();
}




// vec! 简化的定义
#[macro_export]
macro_rules! vec {
    ($( $x: expr ), * ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
