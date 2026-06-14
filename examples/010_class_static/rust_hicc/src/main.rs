use class_static::*;

fn show(s: &hicc_std::string) -> String {
    let cs = unsafe { std::ffi::CStr::from_ptr(s.c_str()) };
    cs.to_str().unwrap().to_string()
}

fn main() {
    println!("before: alive={} next_id={} species={}", alive(), next_id(), show(&species()));

    {
        let mut a = Counter::new();
        a.inc(); a.inc();
        let mut b = Counter::new();
        b.inc();
        println!("a id={} count={}", a.id(), a.count());
        println!("b id={} count={}", b.id(), b.count());
        println!("alive={}", alive());
    }
    println!("after: alive={}", alive());
}
