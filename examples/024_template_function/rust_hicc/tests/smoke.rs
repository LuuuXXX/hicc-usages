use template_function::*;

fn show(s: &hicc_std::string) -> String {
    let cs = unsafe { std::ffi::CStr::from_ptr(s.c_str()) };
    cs.to_str().unwrap().to_string()
}

#[test]
fn template_int_instantiation() {
    assert_eq!(add_int(2, 3), 5);
    assert_eq!(max_of_int(7, 3), 7);
    assert_eq!(show(&describe_int(42)), "value=42");
}

#[test]
fn template_double_instantiation() {
    assert!((add_double(2.5, 3.5) - 6.0).abs() < 0.001);
}
