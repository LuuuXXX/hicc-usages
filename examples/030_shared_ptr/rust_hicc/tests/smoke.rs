use shared_ptr::make_shared_obj;

#[test]
fn shared_ptr_factory_returns_usable_obj() {
    // hicc unwraps shared_ptr<T> to owned T on the Rust side. The C++
    // shared_ptr's refcount is bypassed — the binding treats the result
    // as unique ownership (sufficient for the smoke test).
    let r = make_shared_obj();
    assert!(r.use_count() >= 1);
}
