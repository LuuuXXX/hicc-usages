use custom_deleter::IntArray;

#[test]
fn int_array_via_handle() {
    let arr = IntArray::new(5);
    assert_eq!(arr.size(), 5);
    assert_eq!(arr.read_at(0), 0);
    assert_eq!(arr.read_at(1), 1);
    assert_eq!(arr.read_at(2), 4);
    assert_eq!(arr.read_at(3), 9);
    assert_eq!(arr.read_at(4), 16);
}

#[test]
fn custom_deleter_status_ok() {
    assert_eq!(custom_deleter::custom_deleter_status(), 1);
}
