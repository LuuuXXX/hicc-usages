use union_basic::*;
use std::ffi::CStr;

fn cs(s: &hicc_std::string) -> &str {
    unsafe { CStr::from_ptr(s.c_str()) }.to_str().unwrap_or("<bad utf-8>")
}

#[test]
fn value_accessors() {
    let vi = make_value_int(42);
    assert_eq!(value_as_int(vi), 42);

    let vf = make_value_float(3.14f32);
    assert!((value_as_float(vf) - 3.14f32).abs() < 1e-6);

    let vl = make_value_long(1234567890);
    assert_eq!(value_as_long(vl), 1234567890);
}

#[test]
fn box_constructors_and_tag() {
    let b1 = Box::new_int(7);
    assert_eq!(b1.tag(), Tag::Int);
    assert_eq!(b1.as_int(), 7);

    let b2 = Box::new_float(2.5f32);
    assert_eq!(b2.tag(), Tag::Float);
    assert!((b2.as_float() - 2.5f32).abs() < 1e-6);

    let b3 = Box::new_long(99);
    assert_eq!(b3.tag(), Tag::Long);
    assert_eq!(b3.as_long(), 99);
}

#[test]
fn box_set_and_describe() {
    let mut b = Box::new_int(7);
    assert_eq!(cs(&b.describe()), "int(7)");

    b.set_float(1.5f32);
    assert_eq!(b.tag(), Tag::Float);
    assert!((b.as_float() - 1.5f32).abs() < 1e-6);
    assert_eq!(cs(&b.describe()), "float(1.5)");

    b.set_long(42);
    assert_eq!(b.tag(), Tag::Long);
    assert_eq!(b.as_long(), 42);
    assert_eq!(cs(&b.describe()), "long(42)");
}
