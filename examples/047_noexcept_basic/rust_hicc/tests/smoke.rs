use noexcept_basic::{safe_adder_new};

#[test]
fn noexcept_methods_stripped_for_hicc() {
    let s = safe_adder_new(100);
    assert_eq!(s.add(5), 105);
    assert_eq!(s.sub(5), 95);
    assert_eq!(s.combined(7, 3), 104);
}
