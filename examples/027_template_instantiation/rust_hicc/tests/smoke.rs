use template_instantiation::*;

fn show(s: &hicc_std::string) -> String {
    let cs = unsafe { std::ffi::CStr::from_ptr(s.c_str()) };
    cs.to_str().unwrap().to_string()
}

#[test]
fn pair_int_basic() {
    let mut p = PairInt::new(10, 20);
    assert_eq!(p.first(), 10);
    assert_eq!(p.second(), 20);
    assert_eq!(p.sum(), 30);
    p.swap();
    assert_eq!(p.first(), 20);
}

#[test]
fn pair_string_sum() {
    let a = hicc_std::string::from(c"hello");
    let b = hicc_std::string::from(c"world");
    let ps = PairString::new(&a, &b);
    assert_eq!(show(&ps.sum()), "helloworld");
}
