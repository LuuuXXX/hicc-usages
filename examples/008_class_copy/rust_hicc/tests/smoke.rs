use class_copy::*;

fn show(s: &hicc_std::string) -> String {
    let cs = unsafe { std::ffi::CStr::from_ptr(s.c_str()) };
    cs.to_str().unwrap().to_string()
}

#[test]
fn ctor_with_args() {
    let t = hicc_std::string::from(c"hello");
    let b = Buffer::new(5, &t);
    assert_eq!(b.size(), 5);
    assert_eq!(show(&b.tag()), "hello");
}

#[test]
fn default_ctor() {
    let b = Buffer::default_();
    assert_eq!(b.size(), 0);
    assert_eq!(show(&b.tag()), "empty");
}

#[test]
fn copy_ctor() {
    let t = hicc_std::string::from(c"orig");
    let a = Buffer::new(7, &t);
    let b = a.clone();
    assert_eq!(a.size(), b.size());
}

#[test]
fn move_ctor() {
    let t = hicc_std::string::from(c"movable");
    let a = Buffer::new(3, &t);
    let b = Buffer::move_from(a);
    assert_eq!(b.size(), 3);
}
