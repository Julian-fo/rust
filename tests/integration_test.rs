extern crate app;

#[test]
fn it_adds_two() {
    assert_eq!(4, app::adder::add(2, 2));
}

#[test]
#[should_panic(expected = "Panic!")]
fn will_panic() {
    panic!("Panic!");
}