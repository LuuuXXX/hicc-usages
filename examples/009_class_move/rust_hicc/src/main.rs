use class_move::*;

fn show(s: &hicc_std::string) -> String {
    let cs = unsafe { std::ffi::CStr::from_ptr(s.c_str()) };
    cs.to_str().unwrap().to_string()
}

fn main() {
    let t = hicc_std::string::from(c"A");
    let mut a = Holder::new(3, &t);
    a.add_to(10);
    println!("a size={} first={} tag={}", a.size(), a.first(), show(&a.tag()));

    let b = Holder::move_from(a);
    println!("b size={} (a moved)", b.size());

    let c = Holder::default_();
    println!("c default size={} tag={}", c.size(), show(&c.tag()));
}
