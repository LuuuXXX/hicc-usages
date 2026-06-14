use variadic_functions::{sum2, sum3};

#[test]
fn fixed_arity_sum2() {
    assert_eq!(sum2(10, 20), 30);
    assert_eq!(sum2(-5, 5), 0);
}

#[test]
fn fixed_arity_sum3() {
    assert_eq!(sum3(1, 2, 3), 6);
    assert_eq!(sum3(-1, -2, -3), -6);
}
