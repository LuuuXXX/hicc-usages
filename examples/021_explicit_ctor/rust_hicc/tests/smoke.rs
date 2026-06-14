use explicit_ctor::*;

fn show(s: &hicc_std::string) -> String {
    let cs = unsafe { std::ffi::CStr::from_ptr(s.c_str()) };
    cs.to_str().unwrap().to_string()
}

#[test]
fn distance_explicit_ctors() {
    let mut a = Distance::from_meters(100.5);
    assert!((a.meters() - 100.5).abs() < 0.001);
    let b = Distance::from_m_cm(2, 50);
    assert!((b.meters() - 2.5).abs() < 0.001);
    a.add(&b);
    assert!((a.meters() - 103.0).abs() < 0.001);
}

#[test]
fn wrapper_explicit_ctor() {
    let tag = hicc_std::string::from(c"config");
    let w = Wrapper::new(&tag, 3);
    assert_eq!(show(&w.tag()), "config");
    assert_eq!(w.level(), 3);
}
