use class_static::*;

fn show(s: &hicc_std::string) -> String {
    let cs = unsafe { std::ffi::CStr::from_ptr(s.c_str()) };
    cs.to_str().unwrap().to_string()
}

#[test]
fn static_method_and_instance() {
    let before = alive();
    let mut a = Counter::new();
    a.inc(); a.inc();
    assert_eq!(a.count(), 2);
    assert_eq!(alive(), before + 1);
    assert_eq!(show(&species()), "counter");
}

#[test]
fn static_field_mutation() {
    let prev = total_created();
    add_total(5);
    assert_eq!(total_created(), prev + 5);
    add_total(-5);
}
