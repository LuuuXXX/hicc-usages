use function_overload::{add_f64, add_i32};

#[test]
fn overload_int() {
    assert_eq!(add_i32(2, 3), 5);
    assert_eq!(add_i32(-10, 4), -6);
}

#[test]
fn overload_double() {
    let got = add_f64(1.5, 2.5);
    assert!((got - 4.0).abs() < 1e-9);
}
