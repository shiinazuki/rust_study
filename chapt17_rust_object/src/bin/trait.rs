fn main() {
    let screen = Screen {
        components: vec![
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from({ "Ok" }),
            }),
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
        ],
    };

    screen.run();
}

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!(
            "画了一个 SelectBox height={}, width={}, options={:?}",
            self.height, self.width, self.options
        );
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!(
            "画了一个 Button height={}, width={}, label={}",
            self.height, self.width, self.label
        );
    }
}

// 这与定义使用了带有 trait bound 的泛型类型参数的结构体不同
// trait 对象则允许在运行时替代多种具体类型
// 通过使用 trait 对象的方法，一个 Screen 实例可以存放一个既能包含 Box<Button>，也能包含 Box<TextField> 的 Vec<T>

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    // 在 Screen 上实现一个 run 方法，该方法在每个 component 上调用 draw 方法
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// 泛型类型参数一次只能替代一个具体类型
// 这限制了 Screen 实例必须拥有一个全是 Button 类型或者全是 TextField 类型的组件列表

pub struct Screen_Two<T: Draw> {
    pub components: Vec<T>,
}

impl<T> Screen_Two<T>
where
    T: Draw,
{
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

trait Draw {
    fn draw(&self);
}
