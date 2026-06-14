use virtual_override::*;

fn show(s: &hicc_std::string) -> String {
    let cs = unsafe { std::ffi::CStr::from_ptr(s.c_str()) };
    cs.to_str().unwrap().to_string()
}

#[test]
fn triangle_override_only_sides() {
    let n = hicc_std::string::from(c"tri1");
    let t = Triangle::new(&n);
    assert_eq!(show(&t.name()), "tri1");
    assert_eq!(t.sides(), 3);
    assert_eq!(show(&t.describe()), "tri1/sides=3");
}

#[test]
fn pentagon_override_describe_too() {
    let n = hicc_std::string::from(c"pent1");
    let p = Pentagon::new(&n);
    assert_eq!(p.sides(), 5);
    assert_eq!(show(&p.describe()), "pent1(pentagon)");
}
