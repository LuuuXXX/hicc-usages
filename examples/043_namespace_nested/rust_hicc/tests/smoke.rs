use namespace_nested::{ns_add, ns_combined, ns_mul};

#[test]
fn nested_namespace_via_top_level_wrappers() {
    assert_eq!(ns_add(2, 3), 5);
    assert_eq!(ns_mul(2, 3), 6);
    assert_eq!(ns_combined(1, 2, 3), 6);
}
