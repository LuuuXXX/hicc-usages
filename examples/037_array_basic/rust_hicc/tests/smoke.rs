use array_basic::*;

#[test]
fn array_fill_and_aggregates() {
    let mut a = array5_new();
    fill_array(&mut a, 10);
    assert_eq!(a.size(), 5);
    assert_eq!(a.as_slice(), &[10, 11, 12, 13, 14]);
    assert_eq!(array_sum(&a), 60);
    assert_eq!(array_max(&a), 14);
    assert_eq!(array_avg(&a), 12.0);
}

#[test]
fn array_get_set_via_native_api() {
    let mut a = array5_new();
    fill_array(&mut a, 0);
    assert_eq!(*a.get(0).unwrap(), 0);
    assert_eq!(*a.get(4).unwrap(), 4);
    assert!(a.get(5).is_none());
    // mutate via get_mut
    *a.get_mut(2).unwrap() = 99;
    assert_eq!(*a.get(2).unwrap(), 99);
    assert_eq!(*a.front().unwrap(), 0);
    assert_eq!(*a.back().unwrap(), 4);
}
