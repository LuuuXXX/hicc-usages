use explicit_ctor::*;

fn show(s: &hicc_std::string) -> String {
    let cs = unsafe { std::ffi::CStr::from_ptr(s.c_str()) };
    cs.to_str().unwrap().to_string()
}

fn main() {
    let mut a = Distance::from_meters(100.5);
    let b = Distance::from_m_cm(2, 50);
    a.add(&b);
    println!("a.meters={}", a.meters());

    let tag = hicc_std::string::from(c"config");
    let w = Wrapper::new(&tag, 3);
    println!("w tag={} level={}", show(&w.tag()), w.level());
}
