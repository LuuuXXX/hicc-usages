use namespace_nested::*;
use std::ffi::CStr;

fn cs(s: &hicc_std::string) -> &str {
    unsafe { CStr::from_ptr(s.c_str()) }.to_str().unwrap_or("<bad utf-8>")
}

#[test]
fn n1_n2_n3_foo_and_compute() {
    let mut foo = Foo::new(42);
    assert_eq!(foo.value(), 42);
    assert_eq!(cs(&foo.describe()), "Foo(42)");

    foo.set_value(100);
    assert_eq!(foo.value(), 100);

    assert_eq!(compute(5), 26); // 5*5+1
}

#[test]
fn n1_inner_bar() {
    let mut bar = Bar::new(&hicc_std::string::from(c"hello"));
    assert_eq!(cs(&bar.name()), "hello");
    bar.rename(&hicc_std::string::from(c"world"));
    assert_eq!(cs(&bar.name()), "world");
}

#[test]
fn outer_deep_deeper_free_funcs() {
    assert_eq!(add(3, 4), 7);
    assert_eq!(triple(7), 21);
}
