use variadic_template::*;

fn show(s: &hicc_std::string) -> String {
    let cs = unsafe { std::ffi::CStr::from_ptr(s.c_str()) };
    cs.to_str().unwrap().to_string()
}

fn main() {
    let a = hicc_std::string::from(c"hello");
    let b = hicc_std::string::from(c", ");
    let c = hicc_std::string::from(c"world");
    println!("format_three={}", show(&format_three(&a, &b, &c)));

    println!("sum_two(1,2)={}", sum_two(1, 2));
    println!("sum_three(10,20,30)={}", sum_three(10, 20, 30));
    println!("sum_five(1,2,3,4,5)={}", sum_five(1, 2, 3, 4, 5));
}
