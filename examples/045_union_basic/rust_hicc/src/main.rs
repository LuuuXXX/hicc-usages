use union_basic::*;
use std::ffi::CStr;

fn cs(s: &hicc_std::string) -> &str {
    unsafe { CStr::from_ptr(s.c_str()) }.to_str().unwrap_or("<bad utf-8>")
}

fn main() {
    let vi = make_value_int(42);
    let vf = make_value_float(3.14f32);
    let vl = make_value_long(1234567890);
    println!("value_as_int = {}", value_as_int(vi));
    println!("value_as_float = {}", value_as_float(vf));
    println!("value_as_long = {}", value_as_long(vl));

    let b1 = Box::new_int(7);
    let b2 = Box::new_float(2.5f32);
    let b3 = Box::new_long(99);
    println!("b1 {}", cs(&b1.describe()));
    println!("b2 {}", cs(&b2.describe()));
    println!("b3 {}", cs(&b3.describe()));

    let mut b1m = Box::new_int(7);
    b1m.set_float(1.5f32);
    println!("after set_float b1m {}", cs(&b1m.describe()));

    println!("b1 tag = {:?}", b1.tag());
    println!("b2 tag = {:?}", b2.tag());
    println!("b3 tag = {:?}", b3.tag());
}
