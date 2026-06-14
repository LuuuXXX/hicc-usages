use string_basic::*;

fn show(s: &hicc_std::string) -> String {
    let cs = unsafe { std::ffi::CStr::from_ptr(s.c_str()) };
    cs.to_str().unwrap().to_string()
}

#[test]
fn string_greet_concat_upper() {
    let who = hicc_std::string::from(c"world");
    let g = greet(&who);
    assert_eq!(show(&g), "hello, world!");

    let cat = concat(&hicc_std::string::from(c"foo"), &hicc_std::string::from(c"bar"));
    assert_eq!(show(&cat), "foobar");

    let upper = to_upper(&g);
    assert_eq!(show(&upper), "HELLO, WORLD!");
}

#[test]
fn string_length_contains() {
    let s = hicc_std::string::from(c"hello, world!");
    assert_eq!(string_length(&s), 13);
    assert!(contains_substring(&s, &hicc_std::string::from(c"world")));
    assert!(!contains_substring(&s, &hicc_std::string::from(c"xyz")));
    assert!(contains_substring(&s, &hicc_std::string::from(c"")));  // empty always matches
}
