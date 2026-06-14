use namespace_nested::*;
use std::ffi::CStr;

fn cs(s: &hicc_std::string) -> &str {
    unsafe { CStr::from_ptr(s.c_str()) }.to_str().unwrap_or("<bad utf-8>")
}

fn main() {
    let mut foo = Foo::new(42);
    println!("foo.value() = {}", foo.value());
    println!("foo.describe() = {}", cs(&foo.describe()));
    foo.set_value(100);
    println!("after set foo.value() = {}", foo.value());
    println!("compute(5) = {}", compute(5));

    let mut bar = Bar::new(&hicc_std::string::from(c"hello"));
    println!("bar.name() = {}", cs(&bar.name()));
    bar.rename(&hicc_std::string::from(c"world"));
    println!("after rename bar.name() = {}", cs(&bar.name()));

    println!("add(3,4) = {}", add(3, 4));
    println!("triple(7) = {}", triple(7));
}
