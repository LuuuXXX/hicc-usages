use virtual_basic::*;

fn show(s: &hicc_std::string) -> String {
    let cs = unsafe { std::ffi::CStr::from_ptr(s.c_str()) };
    cs.to_str().unwrap().to_string()
}

#[test]
fn rectangle_virtual_dispatch() {
    let r = Rectangle::new(3.0, 4.0);
    assert_eq!(show(&r.name()), "rect");
    assert!((r.area() - 12.0).abs() < 0.001);
    assert!((r.perimeter() - 14.0).abs() < 0.001);
    assert!(show(&r.describe()).contains("12"));
}

#[test]
fn ellipse_virtual_dispatch() {
    let e = Ellipse::new(2.0, 1.0);
    assert_eq!(show(&e.name()), "ellipse");
    assert!((e.area() - 6.28318).abs() < 0.01);
    assert!(e.perimeter() > 0.0);
}
