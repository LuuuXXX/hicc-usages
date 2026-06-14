use tuple_basic::*;

fn show(s: &hicc_std::string) -> String {
    let cs = unsafe { std::ffi::CStr::from_ptr(s.c_str()) };
    cs.to_str().unwrap().to_string()
}

#[test]
fn tuple_create_and_read_fields() {
    let t = make_triple(7, &hicc_std::string::from(c"alice"), 9.5);
    assert_eq!(triple_id(&t), 7);
    assert_eq!(show(&triple_name(&t)), "alice");
    assert_eq!(triple_score(&t), 9.5);
}

#[test]
fn tuple_set_fields() {
    let mut t = make_triple(1, &hicc_std::string::from(c"bob"), 1.0);
    set_id(&mut t, 100);
    set_score(&mut t, 50.5);
    assert_eq!(triple_id(&t), 100);
    assert_eq!(triple_score(&t), 50.5);
}
