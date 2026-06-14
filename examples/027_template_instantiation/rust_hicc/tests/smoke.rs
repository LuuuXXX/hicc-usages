use template_instantiation::int_stack_new;

#[test]
fn explicit_instantiation_usable() {
    let mut s = int_stack_new();
    assert!(s.empty());
    s.push(42);
    assert!(!s.empty());
    assert_eq!(s.pop(), 42);
    assert!(s.empty());
}
