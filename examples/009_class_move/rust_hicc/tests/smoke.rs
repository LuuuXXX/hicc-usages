use class_move::*;

fn show(s: &hicc_std::string) -> String {
    let cs = unsafe { std::ffi::CStr::from_ptr(s.c_str()) };
    cs.to_str().unwrap().to_string()
}

#[test]
fn move_basic() {
    let t = hicc_std::string::from(c"A");
    let a = Holder::new(3, &t);
    assert_eq!(a.size(), 3);
    assert_eq!(show(&a.tag()), "A");
}

#[test]
fn move_ctor() {
    let t = hicc_std::string::from(c"M");
    let a = Holder::new(2, &t);
    let b = Holder::move_from(a);
    assert_eq!(b.size(), 2);
    // a 被移动，理论上 size=0 但 Rust 不应再访问 a
}

#[test]
fn operator_add_to() {
    let t = hicc_std::string::from(c"A");
    let mut a = Holder::new(3, &t);
    a.add_to(10);
    assert_eq!(a.first(), 10);
}
