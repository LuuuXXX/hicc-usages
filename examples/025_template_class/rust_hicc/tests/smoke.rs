use template_class::*;

fn show(s: &hicc_std::string) -> String {
    let cs = unsafe { std::ffi::CStr::from_ptr(s.c_str()) };
    cs.to_str().unwrap().to_string()
}

#[test]
fn stack_int_basic() {
    let mut s = StackInt::new();
    assert!(s.empty());
    s.push(10); s.push(20); s.push(30);
    assert_eq!(s.size(), 3);
    assert_eq!(s.top(), 30);
    s.pop();
    assert_eq!(s.top(), 20);
}

#[test]
fn stack_string_basic() {
    let mut s = StackString::new();
    let h = hicc_std::string::from(c"hello");
    let w = hicc_std::string::from(c"world");
    s.push(&h); s.push(&w);
    assert_eq!(s.size(), 2);
    assert_eq!(show(&s.top()), "world");
}
