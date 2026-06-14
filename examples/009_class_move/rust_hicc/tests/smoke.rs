use class_move::resource_new;

#[test]
fn consume_moves_self() {
    let r = resource_new(99);
    assert_eq!(r.peek(), 99);
    assert!(r.is_valid());

    // `consume_value` takes self by value — consumes the receiver.
    let v = r.consume_value();
    assert_eq!(v, 99);
}
