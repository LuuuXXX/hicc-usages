use template_function::*;

fn show(s: &hicc_std::string) -> String {
    let cs = unsafe { std::ffi::CStr::from_ptr(s.c_str()) };
    cs.to_str().unwrap().to_string()
}

fn main() {
    println!("add_int(2,3)={}", add_int(2, 3));
    println!("add_double(2.5,3.5)={}", add_double(2.5, 3.5));
    println!("max_of_int(7,3)={}", max_of_int(7, 3));
    println!("describe_int(42)={}", show(&describe_int(42)));
}
