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

fn main() {
    println!("int name={} desc={}", show_cstr(int_name()), show_str(&int_describe(42)));
    println!("double name={} desc={}", show_cstr(double_name()), show_str(&double_describe(3.14)));
    let s = hicc_std::string::from(c"hi");
    println!("string name={} desc={}", show_cstr(string_name()), show_str(&string_describe(&s)));
}
