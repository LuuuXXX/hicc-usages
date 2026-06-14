use inheritance_single::*;

fn show(s: &hicc_std::string) -> String {
    let cs = unsafe { std::ffi::CStr::from_ptr(s.c_str()) };
    cs.to_str().unwrap().to_string()
}

fn main() {
    let n = hicc_std::string::from(c"Rex");
    let d = Dog::new(&n);
    println!("{} says {} legs={} breed={}", show(&d.name()), show(&d.sound()), d.legs(), show(&d.breed()));

    let n2 = hicc_std::string::from(c"Mimi");
    let c = Cat::new(&n2);
    println!("{} says {} legs={}", show(&c.name()), show(&c.sound()), c.legs());
}
