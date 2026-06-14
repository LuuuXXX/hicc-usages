use lambda_basic::{add_then_double, double_it, sum_with_offset};

#[test]
fn lambda_via_named_function() {
    assert_eq!(double_it(21), 42);
    assert_eq!(add_then_double(2, 3), 10);
    let mut arr: [i32; 3] = [1, 2, 3];
    assert_eq!(sum_with_offset(arr.as_mut_ptr(), 3, 10), 1 + 2 + 3 + 10);
}
