use constexpr_basic::*;

#[test]
fn const_static_data() {
    assert!(((*pi()) - 3.14159265358979f64).abs() < 1e-10);
    assert!(((*e_constant()) - 2.71828182845905f64).abs() < 1e-10);
    assert_eq!(*buffer_size(), 256);
    assert_eq!(*max_tries(), 5);
    assert_eq!(*big_number(), 9000000000);
}

#[test]
fn constexpr_free_functions() {
    assert_eq!(square(7), 49);
    assert_eq!(square(0), 0);
    assert_eq!(square(-3), 9);

    assert_eq!(factorial(0), 1);
    assert_eq!(factorial(1), 1);
    assert_eq!(factorial(5), 120);
    assert_eq!(factorial(10), 3628800);
}

#[test]
fn compute_area_via_constexpr_method() {
    let a2 = compute_area(2.0);
    let a3 = compute_area(3.0);
    assert!((a2 - 12.566370614).abs() < 1e-6);
    assert!((a3 - 28.274333882).abs() < 1e-6);
}
