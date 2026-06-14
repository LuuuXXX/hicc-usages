use virtual_override::*;

fn show(s: &hicc_std::string) -> String {
    let cs = unsafe { std::ffi::CStr::from_ptr(s.c_str()) };
    cs.to_str().unwrap().to_string()
}

fn main() {
    let n = hicc_std::string::from(c"tri1");
    let t = Triangle::new(&n);
    println!("{} sides={} desc={}", show(&t.name()), t.sides(), show(&t.describe()));

    let n2 = hicc_std::string::from(c"pent1");
    let p = Pentagon::new(&n2);
    println!("{} sides={} desc={}", show(&p.name()), p.sides(), show(&p.describe()));
}
