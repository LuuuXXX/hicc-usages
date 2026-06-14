use inheritance_multiple::*;

fn show(s: &hicc_std::string) -> String {
    let cs = unsafe { std::ffi::CStr::from_ptr(s.c_str()) };
    cs.to_str().unwrap().to_string()
}

fn main() {
    let c = Circle::new(2.5);
    c.draw();
    println!("{} serialize={} bytes={}", show(&c.shape()), show(&c.serialize()), c.bytes());

    let s = Square::new(4.0);
    s.draw();
    println!("{} serialize={}", show(&s.shape()), show(&s.serialize()));
}
