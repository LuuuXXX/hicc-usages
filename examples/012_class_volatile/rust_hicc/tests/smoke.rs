use class_volatile::*;

#[test]
fn basic_safe_access() {
    let mut s = Sensor::new(7);
    assert_eq!(s.id(), 7);
    assert_eq!(s.safe_read(), 0);
    assert_eq!(s.counter(), 0);
    s.safe_write(123);
    assert_eq!(s.safe_read(), 123);
    assert_eq!(s.counter(), 1);
}

#[test]
fn counter_increments() {
    let mut s = Sensor::new(1);
    for i in 1..=5 {
        s.safe_write(i * 10);
    }
    assert_eq!(s.safe_read(), 50);
    assert_eq!(s.counter(), 5);
}
