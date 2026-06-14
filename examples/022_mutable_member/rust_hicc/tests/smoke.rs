use mutable_member::*;

fn show(s: &hicc_std::string) -> String {
    let cs = unsafe { std::ffi::CStr::from_ptr(s.c_str()) };
    cs.to_str().unwrap().to_string()
}

#[test]
fn mutable_in_const_method() {
    let key = hicc_std::string::from(c"user:1");
    let q = Query::new(&key);
    assert_eq!(show(&q.execute()), "[result for user:1]");
    assert_eq!(show(&q.execute()), "[result for user:1]");
    assert_eq!(q.call_count(), 2);
}
