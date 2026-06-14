//! 039_lambda_basic: lambda expressions
//!
//! hicc 模式：C++ lambda 通过 `std::function<R(Args...)>` 跨 FFI 边界。
//! Rust 端用 `hicc::Function<fn(Args...) -> R>` 接收 / 传递。
//! 闭包 → `hicc::Function` 转换用 `.into()`。
//!
//! 函数签名映射：
//! - C++ `std::function<int(int)>`  -> Rust `hicc::Function<fn(i32) -> i32>`
//! - C++ `std::function<std::string(std::string)>` -> Rust `hicc::Function<fn(hicc_std::string) -> hicc_std::string>`

hicc::cpp! {
    #include "lambda_basic.h"
    #include <hicc/std/string.hpp>
}

hicc::import_lib! {
    #![link_name = "lambda_basic"]

    class string = hicc_std::string;

    #[cpp(func = "int lambda_basic_ns::apply_int(int, std::function<int(int)>)")]
    pub fn apply_int(x: i32, fn_: hicc::Function<fn(i32) -> i32>) -> i32;

    #[cpp(func = "std::function<int(int)> lambda_basic_ns::make_adder(int)")]
    pub fn make_adder(add: i32) -> hicc::Function<fn(i32) -> i32>;

    #[cpp(func = "std::function<int(int)> lambda_basic_ns::compose(std::function<int(int)>, std::function<int(int)>)")]
    pub fn compose(
        f: hicc::Function<fn(i32) -> i32>,
        g: hicc::Function<fn(i32) -> i32>,
    ) -> hicc::Function<fn(i32) -> i32>;

    #[cpp(func = "std::string lambda_basic_ns::shout(std::function<std::string(std::string)>, const std::string&)")]
    pub fn shout(
        fn_: hicc::Function<fn(hicc_std::string) -> hicc_std::string>,
        input: &hicc_std::string,
    ) -> hicc_std::string;
}
