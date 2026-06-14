use class_copy::*;

fn show(s: &hicc_std::string) -> String {
    let cs = unsafe { std::ffi::CStr::from_ptr(s.c_str()) };
    cs.to_str().unwrap().to_string()
}

fn main() {
    let tag = hicc_std::string::from(c"A");
    let a = Buffer::new(10, &tag);
    println!("a: size={} tag={}", a.size(), show(&a.tag()));

    let b = a.clone();
    println!("b (cloned): size={} tag={}", b.size(), show(&b.tag()));

    let c = Buffer::move_from(a); // a 被 move，不可再用
    println!("c (moved): size={} tag={}", c.size(), show(&c.tag()));
}
