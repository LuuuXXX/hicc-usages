use class_basic::counter_new;

#[test]
fn counter_basic() {
    let mut c = counter_new();
    assert_eq!(c.get(), 0);
    c.inc();
    c.inc();
    c.inc();
    assert_eq!(c.get(), 3);
    c.reset();
    assert_eq!(c.get(), 0);
}
