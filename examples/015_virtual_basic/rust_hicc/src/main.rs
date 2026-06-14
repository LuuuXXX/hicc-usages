use virtual_basic::*;

fn show(s: &hicc_std::string) -> String {
    let cs = unsafe { std::ffi::CStr::from_ptr(s.c_str()) };
    cs.to_str().unwrap().to_string()
}

fn main() {
    let r = Rectangle::new(3.0, 4.0);
    println!("{} area={} perimeter={} desc={}", show(&r.name()), r.area(), r.perimeter(), show(&r.describe()));

    let e = Ellipse::new(2.0, 1.0);
    println!("{} area={} perimeter={}", show(&e.name()), e.area(), e.perimeter());
}
