use shared_ptr::*;

#[test]
fn shared_ptr_factory_and_clone() {
    let c1 = make_counter(10);
    assert_eq!(c1.get().value(), 10);
    assert_eq!(use_count(&c1), 1);

    let c2 = clone_counter(&c1);
    assert_eq!(use_count(&c1), 2);

    c2.get().increment();
    assert_eq!(c1.get().value(), 11);
}

#[test]
fn shared_ptr_decrement_drops_on_zero() {
    let c1 = make_counter(100);
    assert_eq!(use_count(&c1), 1);

    {
        let _c2 = clone_counter(&c1);
        assert_eq!(use_count(&c1), 2);
    } // _c2 离开作用域，引用计数减回 1

    assert_eq!(use_count(&c1), 1);
    assert_eq!(c1.get().value(), 100);
}
