use std::ffi::CStr;
use virtual_override::{error_logger_new, info_logger_new, string_new};

#[test]
fn override_dispatches_correctly() {
    let i = info_logger_new();
    let e = error_logger_new();

    let msg = string_new(b"hello\0".as_ptr() as *const i8);
    let info_out = i.format(&msg);
    let err_out = e.format(&msg);

    unsafe {
        assert_eq!(CStr::from_ptr(info_out.c_str()).to_bytes(), b"[INFO] hello");
        assert_eq!(CStr::from_ptr(err_out.c_str()).to_bytes(),  b"[ERROR] hello");
    }
}
