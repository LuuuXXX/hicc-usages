use std::ffi::CStr;
use template_specialization::{type_name_bool, type_name_generic, type_name_int};

#[test]
fn specialization_dispatches_correctly() {
    let i = unsafe { CStr::from_ptr(type_name_int()).to_bytes() };
    let b = unsafe { CStr::from_ptr(type_name_bool()).to_bytes() };
    let g = unsafe { CStr::from_ptr(type_name_generic()).to_bytes() };
    assert_eq!(i, b"int");
    assert_eq!(b, b"bool");
    assert_eq!(g, b"generic");
}
