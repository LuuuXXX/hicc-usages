use function_overload::*;

#[test]
fn int_overload() { assert_eq!(add_int(10, 20), 30); }

#[test]
fn double_overload() { assert!((add_double(1.5, 2.5) - 4.0).abs() < 1e-9); }

#[test]
fn three_arg_overload() { assert_eq!(add_three(1, 2, 3), 6); }

#[test]
fn string_overload() {
    let a = hicc_std::string::from(c"foo");
    let b = hicc_std::string::from(c"bar");
    let r = add_string(&a, &b);
    let cs = unsafe { std::ffi::CStr::from_ptr(r.c_str()) };
    assert_eq!(cs.to_str().unwrap(), "foobar");
}
