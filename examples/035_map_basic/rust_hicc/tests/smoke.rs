use map_basic::*;

fn show(s: &hicc_std::string) -> String {
    let cs = unsafe { std::ffi::CStr::from_ptr(s.c_str()) };
    cs.to_str().unwrap().to_string()
}

#[test]
fn map_put_size_get_or() {
    let mut m = map_new();
    put(&mut m, 1, &hicc_std::string::from(c"one"));
    put(&mut m, 2, &hicc_std::string::from(c"two"));
    assert_eq!(map_size(&m), 2);
    assert_eq!(show(&get_or(&m, 1, &hicc_std::string::from(c"?"))), "one");
    assert_eq!(show(&get_or(&m, 99, &hicc_std::string::from(c"miss"))), "miss");
    assert_eq!(sum_key_values(&m), 3);
}

#[test]
fn map_native_insert_get() {
    let mut m = map_new();
    let v10 = hicc_std::string::from(c"ten");
    m.insert(&10, &v10);
    assert_eq!(map_size(&m), 1);
    let v = m.get(&10);
    assert!(v.is_some());
    assert_eq!(show(&v.unwrap()), "ten");
    assert!(m.get(&20).is_none());
}
