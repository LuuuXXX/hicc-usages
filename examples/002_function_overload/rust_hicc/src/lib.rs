//! 002_function_overload: C++ 函数重载 → Rust 端改名导出。
//!
//! hicc 模式：`import_lib!` 多个 `#[cpp(func = ...)]`，Rust 端给每个重载起独立名字。

hicc::cpp! {
    #include "function_overload.h"
    #include <hicc/std/string.hpp>
}

hicc::import_lib! {
    #![link_name = "function_overload"]

    class overload_ns;
    class string = hicc_std::string;

    #[cpp(func = "int overload_ns::add(int, int)")]
    pub fn add_int(a: i32, b: i32) -> i32;

    #[cpp(func = "double overload_ns::add(double, double)")]
    pub fn add_double(a: f64, b: f64) -> f64;

    #[cpp(func = "std::string overload_ns::add(const std::string&, const std::string&)")]
    pub fn add_string(a: &string, b: &string) -> string;

    #[cpp(func = "int overload_ns::add(int, int, int)")]
    pub fn add_three(a: i32, b: i32, c: i32) -> i32;
}
