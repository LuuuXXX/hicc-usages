use operator_overload::{vec2_add, vec2_eq, vec2_new, vec2_sub};

#[test]
fn operator_via_named_wrapper() {
    let a = vec2_new(1, 2);
    let b = vec2_new(3, 4);

    let c = vec2_add(&a, &b);
    assert_eq!(c.x(), 4);
    assert_eq!(c.y(), 6);

    let d = vec2_sub(&b, &a);
    assert_eq!(d.x(), 2);
    assert_eq!(d.y(), 2);

    assert!(!vec2_eq(&a, &b));
    assert!(vec2_eq(&a, &vec2_new(1, 2)));
}
