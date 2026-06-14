use string_basic::*;

fn show(s: &hicc_std::string) -> String {
    let cs = unsafe { std::ffi::CStr::from_ptr(s.c_str()) };
    cs.to_str().unwrap().to_string()
}

fn main() {
    let who = hicc_std::string::from(c"world");
    let g = greet(&who);
    println!("greet    = {}", show(&g));
    println!("upper    = {}", show(&to_upper(&g)));
    println!("concat   = {}", show(&concat(&hicc_std::string::from(c"foo"), &hicc_std::string::from(c"bar"))));
    println!("length   = {}", string_length(&g));
    println!("contains = {}", contains_substring(&g, &hicc_std::string::from(c"world")));
    println!("contains = {}", contains_substring(&g, &hicc_std::string::from(c"xyz")));
}
