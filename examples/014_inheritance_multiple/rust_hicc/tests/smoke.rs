use inheritance_multiple::*;

fn show(s: &hicc_std::string) -> String {
    let cs = unsafe { std::ffi::CStr::from_ptr(s.c_str()) };
    cs.to_str().unwrap().to_string()
}

#[test]
fn circle_inherited_methods() {
    let c = Circle::new(2.5);
    assert_eq!(show(&c.shape()), "Circle");
    assert_eq!(c.bytes(), 12);
    assert!((c.radius() - 2.5).abs() < 0.001);
    assert!(show(&c.serialize()).contains("2.5"));
}

#[test]
fn square_inherited_methods() {
    let s = Square::new(4.0);
    assert_eq!(show(&s.shape()), "Square");
    assert!((s.side() - 4.0).abs() < 0.001);
}
