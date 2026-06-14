use std_function::{add_op, compose_then_add_then_mul, mul_op, run_binary_op};

#[test]
fn std_function_via_named_wrappers() {
    assert_eq!(add_op(2, 3), 5);
    assert_eq!(mul_op(2, 3), 6);
    assert_eq!(run_binary_op(2, 3, 0), 5);
    assert_eq!(run_binary_op(2, 3, 1), 6);
    assert_eq!(run_binary_op(2, 3, 9), -1);
    assert_eq!(compose_then_add_then_mul(2, 10, 3), (2 + 10) * 3);
}
