use std::ffi::CStr;

use hello_world::{add, hello, string_new};

#[test]
fn add_works() {
    assert_eq!(add(2, 3), 5);
    assert_eq!(add(-1, 1), 0);
}

#[test]
fn hello_roundtrip() {
    let who = unsafe { string_new(b"hicc\0".as_ptr() as *const i8) };
    let result = hello(&who);
    let cstr = unsafe { CStr::from_ptr(result.c_str()) };
    assert_eq!(cstr.to_bytes(), b"hello world from hicc!");
}
