use variadic_template::{sum2, sum3, sum4};

#[test]
fn variadic_via_fixed_arity_wrappers() {
    assert_eq!(sum2(1, 2), 3);
    assert_eq!(sum3(1, 2, 3), 6);
    assert_eq!(sum4(1, 2, 3, 4), 10);
}
