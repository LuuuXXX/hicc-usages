use class_volatile::vcounter_new;

#[test]
fn volatile_inc_and_get() {
    let mut c = vcounter_new();
    assert_eq!(c.get(), 0);
    c.inc(); c.inc();
    assert_eq!(c.get(), 2);
    c.reset();
    assert_eq!(c.get(), 0);
}
