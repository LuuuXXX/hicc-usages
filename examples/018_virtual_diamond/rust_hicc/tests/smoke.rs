use virtual_diamond::console_new;

#[test]
fn diamond_simplified_to_single_class() {
    let mut c = console_new();
    assert_eq!(c.priority(), 5);
    c.write(42);
    assert_eq!(c.read(), 42);
    assert_eq!(c.read(), 0);  // cleared after read
}
