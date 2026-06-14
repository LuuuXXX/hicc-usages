use std::ffi::CStr;
use tuple_basic::triple_new;

#[test]
fn tuple_via_named_accessors() {
    let t = triple_new(42, b"hello\0".as_ptr() as *const i8, 3.14);
    assert_eq!(t.first(), 42);
    let s = t.second();
    unsafe {
        assert_eq!(CStr::from_ptr(s.c_str()).to_bytes(), b"hello");
    }
    assert!((t.third() - 3.14).abs() < 1e-9);
}
