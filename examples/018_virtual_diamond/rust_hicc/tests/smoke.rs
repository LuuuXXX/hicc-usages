use virtual_diamond::*;

fn show(s: &hicc_std::string) -> String {
    let cs = unsafe { std::ffi::CStr::from_ptr(s.c_str()) };
    cs.to_str().unwrap().to_string()
}

#[test]
fn diamond_virtual_base_id() {
    let id = hicc_std::string::from(c"dev1");
    let mut c = IOCombo::new(&id);
    assert_eq!(show(&c.id()), "dev1");
    assert_eq!(show(&c.category()), "IOCombo");
    c.write(99);
    assert_eq!(c.last_output(), 99);
    assert_eq!(c.read(), 42);
}

#[test]
fn diamond_virtual_base_shared() {
    let id = hicc_std::string::from(c"shared");
    let c = IOCombo::new(&id);
    assert_eq!(show(&c.id()), "shared");
}
