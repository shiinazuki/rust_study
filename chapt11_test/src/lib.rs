#![feature(duration_constructors)]

pub fn add_two2(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}



struct Rectangle {
    width: u32,
    heigth: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.heigth > other.heigth
    }
}

pub fn add(x: usize, y: usize) -> usize {
    x + y
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello!")
}

struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
      if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {value}."
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {value}."
            );
        }
        Guess {
            value
        }
    }
}



#[cfg(test)]
mod tests {
    use std::{thread, time::{self, Duration}};

    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }

    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn greeter_than_100() {
        Guess::new(200);
    }


    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("H"),
            "Greeting did not contain name, value was '{}':",
            result
        );
    }

        #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4{
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }


    #[test]
    fn it_add_two() {
        assert_eq!(add_two(3), 5);
    }

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(4, result);
    }

    #[test]
    fn another() {
        // panic!("Make this test fail");
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            heigth: 7,
        };

        let smaller = Rectangle {
            width: 5,
            heigth: 1,
        };

        assert!(larger.can_hold(&smaller));
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    #[ignore]
    fn xpensive_test() {
        thread::sleep(time::Duration::from_hours(10));
    }
}
