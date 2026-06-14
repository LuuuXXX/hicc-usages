use variadic_functions::*;

#[test]
fn sum_ints_works() {
    let total = unsafe { sum_ints()(4, 1, 2, 3, 4) };
    assert_eq!(total, 10);
}

#[test]
fn sum_ints_one() {
    let total = unsafe { sum_ints()(1, 42) };
    assert_eq!(total, 42);
}
