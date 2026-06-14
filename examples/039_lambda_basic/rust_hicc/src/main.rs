use lambda_basic::*;

fn show(s: &hicc_std::string) -> String {
    let cs = unsafe { std::ffi::CStr::from_ptr(s.c_str()) };
    cs.to_str().unwrap().to_string()
}

fn main() {
    // 1. 传一个 Rust 闭包给 C++
    let dbl_fn: hicc::Function<fn(i32) -> i32> = (|v: i32| v * 2).into();
    println!("apply_int(5, x*2) = {}", apply_int(5, dbl_fn));

    // 2. 接收 C++ lambda（adder）并调用
    let add10_fn = make_adder(10);
    let add10 = add10_fn.into();
    println!("add10(7) = {}", add10(7));

    // 3. 组合
    let mul2_fn: hicc::Function<fn(i32) -> i32> = (|v: i32| v * 2).into();
    let add5_fn = make_adder(5);
    let pipe = compose(mul2_fn, add5_fn);
    let pipe_cl = pipe.into();
    println!("compose(x*2, x+5)(3) = {}", pipe_cl(3));

    // 4. Rust → C++ 方向的字符串 lambda 较为棘手（在闭包内构造 hicc_std::string
    //    并返回）。standalone 中由 C++ 端演示；此处我们传一个简单的直通 identity，
    //    依赖 C++ 端添加 '!' 后缀。为保证二进制安全而跳过；算术 lambda 见测试。
}
