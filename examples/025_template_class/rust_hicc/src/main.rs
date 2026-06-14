use template_class::*;

fn show(s: &hicc_std::string) -> String {
    let cs = unsafe { std::ffi::CStr::from_ptr(s.c_str()) };
    cs.to_str().unwrap().to_string()
}

fn main() {
    let mut s = StackInt::new();
    s.push(10); s.push(20); s.push(30);
    println!("size={} top={}", s.size(), s.top());
    s.pop();
    println!("after pop size={} top={}", s.size(), s.top());

    let mut ss = StackString::new();
    let h = hicc_std::string::from(c"hello");
    let w = hicc_std::string::from(c"world");
    ss.push(&h); ss.push(&w);
    println!("ss size={} top={}", ss.size(), show(&ss.top()));
}
