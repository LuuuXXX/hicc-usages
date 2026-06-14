use std::ffi::CStr;
use virtual_pure::{mem_storage_new, string_new};

#[test]
fn put_and_get() {
    let mut s = mem_storage_new();
    let k = string_new(b"foo\0".as_ptr() as *const i8);
    let v = string_new(b"bar\0".as_ptr() as *const i8);
    s.put(&k, &v);
    assert_eq!(s.size(), 1);

    let got = s.get(&k);
    unsafe {
        assert_eq!(CStr::from_ptr(got.c_str()).to_bytes(), b"bar");
    }
}
