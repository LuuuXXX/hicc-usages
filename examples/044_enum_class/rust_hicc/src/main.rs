use enum_class::*;
use std::ffi::CStr;

fn cs(s: &hicc_std::string) -> &str {
    unsafe { CStr::from_ptr(s.c_str()) }.to_str().unwrap_or("<bad utf-8>")
}

fn main() {
    let c = Color::Green;
    println!("color_to_int(Green) = {}", color_to_int(c));
    println!("color_name(Green) = {}", cs(&color_name(c)));

    let c2 = color_from_int(2);
    println!("color_from_int(2) name = {}", cs(&color_name(c2)));

    let s = Status::Pending;
    println!("status_to_int(Pending) = {}", status_to_int(s));

    let mut l = Light::new(Color::Red);
    println!("light.current name = {}", cs(&color_name(l.current())));
    println!("light.brightness = {}", l.brightness());
    l.set(Color::Blue);
    println!("after set light.brightness = {}", l.brightness());
    println!("after set light.current name = {}", cs(&color_name(l.current())));
}
