use array_basic::int_array4_new;

#[test]
fn array_set_get_sum() {
    let mut a = int_array4_new();
    assert_eq!(a.size(), 4);
    for i in 0..4 {
        a.set(i, (i as i32) + 1);
    }
    assert_eq!(a.get(0), 1);
    assert_eq!(a.get(3), 4);
    assert_eq!(a.sum(), 1 + 2 + 3 + 4);
}
