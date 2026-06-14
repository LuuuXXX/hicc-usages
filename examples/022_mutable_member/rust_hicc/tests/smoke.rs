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

#[test]
fn mutable_independent_per_instance() {
    // mutable 字段是实例级别，两个 Query 各自计数
    let k1 = hicc_std::string::from(c"a");
    let k2 = hicc_std::string::from(c"b");
    let q1 = Query::new(&k1);
    let q2 = Query::new(&k2);
    assert_eq!(show(&q1.execute()), "[result for a]");
    assert_eq!(show(&q2.execute()), "[result for b]");
    assert_eq!(show(&q2.execute()), "[result for b]");
    assert_eq!(q1.call_count(), 1);
    assert_eq!(q2.call_count(), 2);
}
