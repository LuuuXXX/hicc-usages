use raii_pattern::lock_new;

#[test]
fn raii_via_drop() {
    let l = lock_new(7);
    assert_eq!(l.id(), 7);
    assert!(l.is_locked());
    // Drop fires lock_free → ~Lock() releases.
}
