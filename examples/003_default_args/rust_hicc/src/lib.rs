//! 003_default_args: C++ 缺省参数 → Rust 端省略尾部参数。
//!
//! hicc 模式（reference.md §"忽略C++函数的缺省参数"）：
//! - `#[cpp(func = "...")]` 写**完整** C++ 签名（含所有参数类型）
//! - Rust 函数省略带缺省值的尾部参数

hicc::cpp! {
    #include "default_args.h"
    #include <hicc/std/string.hpp>
}

hicc::import_lib! {
    #![link_name = "default_args"]

    class default_args_ns;
    class string = hicc_std::string;

    // C++ 端签名：int greet(const std::string&, int times = 1, const std::string& suffix = "!")
    #[cpp(func = "int default_args_ns::greet(const std::string&, int, const std::string&)")]
    pub fn greet_full(name: &string, times: i32, suffix: &string) -> i32;

    #[cpp(func = "int default_args_ns::greet(const std::string&, int, const std::string&)")]
    pub fn greet_times(name: &string, times: i32) -> i32;

    #[cpp(func = "int default_args_ns::greet(const std::string&, int, const std::string&)")]
    pub fn greet_default(name: &string) -> i32;

    // C++ 端签名：int compute(int a, int b = 10, int c = 100)
    #[cpp(func = "int default_args_ns::compute(int, int, int)")]
    pub fn compute_full(a: i32, b: i32, c: i32) -> i32;

    #[cpp(func = "int default_args_ns::compute(int, int, int)")]
    pub fn compute_two(a: i32, b: i32) -> i32;

    #[cpp(func = "int default_args_ns::compute(int, int, int)")]
    pub fn compute_one(a: i32) -> i32;
}
