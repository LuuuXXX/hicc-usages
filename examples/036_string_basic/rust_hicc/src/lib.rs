//! 036_string_basic: std::string
//!
//! hicc 模式：直接绑业务函数（参数 / 返回值类型为 std::string）。
//! Rust 端用 `hicc_std::string` 接收。hicc-std 已内置 std::string 的实例化构造
//! （`hicc_std::string::from(c"...")`），不需要手动 typedef。
//!
//! 借助 `class string = hicc_std::string;` 在 import_lib! 中开放别名，
//! 让 `&hicc_std::string` / `*const i8` 互转更顺畅（hicc-std::string 已实现
//! ClassRef 转换）。

hicc::cpp! {
    #include "string_basic.h"
    #include <hicc/std/string.hpp>
}

hicc::import_lib! {
    #![link_name = "string_basic"]

    class string = hicc_std::string;

    #[cpp(func = "std::string string_basic_ns::greet(const std::string&)")]
    pub fn greet(name: &hicc_std::string) -> hicc_std::string;

    #[cpp(func = "std::string string_basic_ns::to_upper(const std::string&)")]
    pub fn to_upper(s: &hicc_std::string) -> hicc_std::string;

    #[cpp(func = "std::string string_basic_ns::concat(const std::string&, const std::string&)")]
    pub fn concat(a: &hicc_std::string, b: &hicc_std::string) -> hicc_std::string;

    #[cpp(func = "size_t string_basic_ns::string_length(const std::string&)")]
    pub fn string_length(s: &hicc_std::string) -> usize;

    #[cpp(func = "bool string_basic_ns::contains_substring(const std::string&, const std::string&)")]
    pub fn contains_substring(hay: &hicc_std::string, needle: &hicc_std::string) -> bool;
}
