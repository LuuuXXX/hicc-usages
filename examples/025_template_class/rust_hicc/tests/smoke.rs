use template_class::{box_double_new, box_int_new};

#[test]
fn template_class_typedef_factory() {
    let mut bi = box_int_new(42);
    assert_eq!(bi.get(), 42);
    bi.set(100);
    assert_eq!(bi.get(), 100);

    let mut bd = box_double_new(3.14);
    assert!((bd.get() - 3.14).abs() < 1e-9);
    bd.set(2.71);
    assert!((bd.get() - 2.71).abs() < 1e-9);
}
