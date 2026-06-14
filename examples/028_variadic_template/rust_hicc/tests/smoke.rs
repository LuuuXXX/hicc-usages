use variadic_template::*;

fn show(s: &hicc_std::string) -> String {
    let cs = unsafe { std::ffi::CStr::from_ptr(s.c_str()) };
    cs.to_str().unwrap().to_string()
}

#[test]
fn variadic_sum_bridges() {
    assert_eq!(sum_two(1, 2), 3);
    assert_eq!(sum_three(10, 20, 30), 60);
    assert_eq!(sum_five(1, 2, 3, 4, 5), 15);
}

#[test]
fn variadic_format_bridge() {
    let a = hicc_std::string::from(c"a");
    let b = hicc_std::string::from(c"b");
    let c = hicc_std::string::from(c"c");
    assert_eq!(show(&format_three(&a, &b, &c)), "abc");
}
