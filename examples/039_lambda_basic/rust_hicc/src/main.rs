use lambda_basic::*;

fn show(s: &hicc_std::string) -> String {
    let cs = unsafe { std::ffi::CStr::from_ptr(s.c_str()) };
    cs.to_str().unwrap().to_string()
}

fn main() {
    // 1. Pass a Rust closure to C++
    let dbl_fn: hicc::Function<fn(i32) -> i32> = (|v: i32| v * 2).into();
    println!("apply_int(5, x*2) = {}", apply_int(5, dbl_fn));

    // 2. Receive a C++ lambda (adder) and call it
    let add10_fn = make_adder(10);
    let add10 = add10_fn.into();
    println!("add10(7) = {}", add10(7));

    // 3. Compose
    let mul2_fn: hicc::Function<fn(i32) -> i32> = (|v: i32| v * 2).into();
    let add5_fn = make_adder(5);
    let pipe = compose(mul2_fn, add5_fn);
    let pipe_cl = pipe.into();
    println!("compose(x*2, x+5)(3) = {}", pipe_cl(3));

    // 4. String lambda is tricky in Rust → C++ direction (constructing hicc_std::string
    //    inside a closure and returning it). Demonstrated via C++ in standalone; here we
    //    pass a simple passthrough-free identity and rely on C++ to add the '!' suffix.
    //    Skipping to keep binary safe; see tests for arithmetic lambda coverage.
}
