use union_basic::value_box_new;

#[test]
fn union_via_value_box_wrapper() {
    let mut b = value_box_new();
    b.set_int(42);
    assert_eq!(b.tag(), 0);
    assert_eq!(b.get_int(), 42);

    b.set_float(3.5);
    assert_eq!(b.tag(), 1);
    assert_eq!(b.get_float(), 3.5);
}
