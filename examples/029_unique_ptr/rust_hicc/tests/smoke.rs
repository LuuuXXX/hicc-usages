use unique_ptr::*;

fn show(s: &hicc_std::string) -> String {
    let cs = unsafe { std::ffi::CStr::from_ptr(s.c_str()) };
    cs.to_str().unwrap().to_string()
}

#[test]
fn unique_ptr_factory_and_drop() {
    let n = hicc_std::string::from(c"res1");
    let r = Resource::new(1, &n);
    assert_eq!(r.id(), 1);
    assert_eq!(show(&r.name()), "res1");

    // Drop 调用 C++ 析构（等价于 unique_ptr 释放）
    let id = r.id();
    drop(r);
    assert_eq!(id, 1);
}

#[test]
fn unique_ptr_field_accessors() {
    let n = hicc_std::string::from(c"database-conn");
    let r = Resource::new(42, &n);
    assert_eq!(r.id(), 42);
    assert_eq!(show(&r.name()), "database-conn");
    // 工厂返回的 Resource 在 Rust 端独占所有权（unique_ptr 语义）
    // 多次访问同一字段返回一致结果
    assert_eq!(r.id(), 42);
    assert_eq!(show(&r.name()), "database-conn");
}
