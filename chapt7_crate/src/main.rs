use rand::Rng;
pub mod garden;

use crate::garden::vegetables;
fn main() {
    vegetables::print();

    let random = rand::thread_rng().gen_range(1..=100);
    println!("{}", random);
}