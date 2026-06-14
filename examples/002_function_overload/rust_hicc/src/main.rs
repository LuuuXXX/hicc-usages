use function_overload::*;
use hicc::AbiClass;

fn main() {
    println!("add_int(1, 2) = {}", add_int(1, 2));
    println!("add_double(1.5, 2.5) = {}", add_double(1.5, 2.5));
    println!("add_three(1, 2, 3) = {}", add_three(1, 2, 3));

    let a = hicc_std::string::from(c"foo");
    let b = hicc_std::string::from(c"bar");
    let result = add_string(&a, &b);
    let cs = unsafe { std::ffi::CStr::from_ptr(result.c_str()) };
    println!("add_string(foo, bar) = {}", cs.to_str().unwrap());
    let _ = result; // 持有所有权直到结束
}
