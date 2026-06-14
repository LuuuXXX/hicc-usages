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
