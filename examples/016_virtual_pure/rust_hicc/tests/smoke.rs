use virtual_pure::*;

fn show(s: &hicc_std::string) -> String {
    let cs = unsafe { std::ffi::CStr::from_ptr(s.c_str()) };
    cs.to_str().unwrap().to_string()
}

#[test]
fn storage_put_get_remove() {
    let mut s = InMemoryStorage::new();
    let k = hicc_std::string::from(c"key1");
    let v = hicc_std::string::from(c"val1");
    assert!(s.put(&k, &v));
    assert_eq!(s.size(), 1);
    assert_eq!(show(&s.get(&k)), "val1");

    assert!(s.remove(&k));
    assert_eq!(s.size(), 0);
    assert_eq!(show(&s.get(&k)), "");
}

#[test]
fn storage_missing_key() {
    let s = InMemoryStorage::new();
    let k = hicc_std::string::from(c"missing");
    assert_eq!(show(&s.get(&k)), "");
    assert_eq!(s.size(), 0);
}
