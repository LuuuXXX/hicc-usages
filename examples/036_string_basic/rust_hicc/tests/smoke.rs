use std::ffi::CStr;
use string_basic::{concat, length, string_new, upper};

#[test]
fn string_roundtrip() {
    let a = string_new(b"hello, \0".as_ptr() as *const i8);
    let b = string_new(b"world\0".as_ptr() as *const i8);

    let c = concat(&a, &b);
    unsafe {
        assert_eq!(CStr::from_ptr(c.c_str()).to_bytes(), b"hello, world");
    }
    assert_eq!(length(&a), 7);

    let u = upper(&string_new(b"abc\0".as_ptr() as *const i8));
    unsafe {
        assert_eq!(CStr::from_ptr(u.c_str()).to_bytes(), b"ABC");
    }
}
