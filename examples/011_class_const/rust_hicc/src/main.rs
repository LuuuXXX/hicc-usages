use class_const::*;

fn show(s: &hicc_std::string) -> String {
    let cs = unsafe { std::ffi::CStr::from_ptr(s.c_str()) };
    cs.to_str().unwrap().to_string()
}

fn main() {
    let t = Temperature::new(25.0);
    println!("value={} unit={} fahrenheit={}", t.value(), show(&t.unit()), t.to_fahrenheit());

    let u = hicc_std::string::from(c"F");
    let t2 = Temperature::new_with_unit(77.0, &u);
    println!("t2 value={} unit={}", t2.value(), show(&t2.unit()));
}
