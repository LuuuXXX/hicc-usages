use virtual_diamond::*;

fn show(s: &hicc_std::string) -> String {
    let cs = unsafe { std::ffi::CStr::from_ptr(s.c_str()) };
    cs.to_str().unwrap().to_string()
}

fn main() {
    let id = hicc_std::string::from(c"dev1");
    let mut c = IOCombo::new(&id);
    c.write(99);
    println!("{} category={} read={} last_out={}",
        show(&c.id()), show(&c.category()), c.read(), c.last_output());
}
