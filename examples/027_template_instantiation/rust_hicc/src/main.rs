use template_instantiation::*;

fn show(s: &hicc_std::string) -> String {
    let cs = unsafe { std::ffi::CStr::from_ptr(s.c_str()) };
    cs.to_str().unwrap().to_string()
}

fn main() {
    let mut p = PairInt::new(10, 20);
    println!("first={} second={} sum={}", p.first(), p.second(), p.sum());
    p.swap();
    println!("after swap first={}", p.first());

    let a = hicc_std::string::from(c"hello");
    let b = hicc_std::string::from(c"world");
    let ps = PairString::new(&a, &b);
    println!("ps sum={}", show(&ps.sum()));
}
