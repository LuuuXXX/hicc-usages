use typeid_rtti::*;

#[test]
fn rtti_same_type() {
    let a1 = DerivedA::new();
    let a2 = DerivedA::new();
    let b = DerivedB::new();
    assert!(same_type_a_a(&a1, &a2));
    assert!(!same_type_a_b(&a1, &b));
}

#[test]
fn rtti_is_derived_a() {
    let a = DerivedA::new();
    let b = DerivedB::new();
    assert!(is_derived_a_a(&a));
    assert!(!is_derived_a_b(&b));
}

#[test]
fn rtti_type_name_nonnull() {
    let a = DerivedA::new();
    let p = type_name_base_a(&a);
    assert!(!p.is_null());
}
