use unique_ptr::*;

fn show(s: &hicc_std::string) -> String {
    let cs = unsafe { std::ffi::CStr::from_ptr(s.c_str()) };
    cs.to_str().unwrap().to_string()
}

fn main() {
    let n = hicc_std::string::from(c"res1");
    let r = Resource::new(1, &n);
    println!("r id={} name={}", r.id(), show(&r.name()));

    // consume: Resource 离开作用域时，Rust Drop 调用 C++ 析构（等价于 unique_ptr 释放）
    let id = r.id();
    drop(r);
    println!("consumed id={} (r dropped)", id);
}
