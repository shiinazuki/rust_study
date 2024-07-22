use chapt11_test::add_two2;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, add_two2(2));
}