use class_copy::{box_clone, box_new};

#[test]
fn copy_is_independent() {
    let mut a = box_new(42);
    let b = box_clone(&a);
    a.set(100);
    assert_eq!(a.get(), 100);  // original modified
    assert_eq!(b.get(), 42);   // clone unchanged
}
