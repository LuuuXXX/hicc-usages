use template_specialization::*;

fn show_cstr(p: *const i8) -> String {
    if p.is_null() { return "<null>".into(); }
    let cs = unsafe { std::ffi::CStr::from_ptr(p) };
    cs.to_str().unwrap().to_string()
}

fn show_str(s: &hicc_std::string) -> String {
    let cs = unsafe { std::ffi::CStr::from_ptr(s.c_str()) };
    cs.to_str().unwrap().to_string()
}

#[test]
fn specialization_int() {
    assert_eq!(show_cstr(int_name()), "int");
    assert_eq!(show_str(&int_describe(42)), "int(42)");
}

#[test]
fn specialization_double_and_string() {
    assert_eq!(show_cstr(double_name()), "double");
    assert_eq!(show_cstr(string_name()), "string");
    let s = hicc_std::string::from(c"hi");
    assert_eq!(show_str(&string_describe(&s)), "string(hi)");
}
