use class_const::*;

fn show(s: &hicc_std::string) -> String {
    let cs = unsafe { std::ffi::CStr::from_ptr(s.c_str()) };
    cs.to_str().unwrap().to_string()
}

#[test]
fn const_read_and_mutate() {
    let t = Temperature::new(25.0);
    assert_eq!(t.value(), 25.0);
    assert_eq!(show(&t.unit()), "C");
    assert!((t.to_fahrenheit() - 77.0).abs() < 0.001);
}

#[test]
fn convert_to_f() {
    let mut t = Temperature::new(0.0);
    let f = hicc_std::string::from(c"F");
    t.convert_to(&f);
    assert!((t.value() - 32.0).abs() < 0.001);
    assert_eq!(show(&t.unit()), "F");
}

#[test]
fn set_value_and_unit() {
    let u = hicc_std::string::from(c"F");
    let mut t = Temperature::new_with_unit(50.0, &u);
    t.set_value(70.0);
    assert_eq!(t.value(), 70.0);
    assert_eq!(show(&t.unit()), "F");
}
