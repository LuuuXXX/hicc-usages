//! 028_variadic_template: 变参模板
//!
//! hicc 模式：变参模板不能直接绑。C++ 端为每个常用 arity 写一个**具签名的桥接函数**
//! （如 `sum_three(int, int, int)` 内部调 `sum_all(...)`），Rust 端按普通函数绑。
//! 这是变参模板唯一可行的 FFI 路径。

hicc::cpp! {
    #include "variadic_template.h"
    #include <hicc/std/string.hpp>

    // 为变参模板桥接固定 arity 的入口
    inline std::string format_three(const std::string& a, const std::string& b, const std::string& c) {
        return variadic_template_ns::format(a, b, c);
    }
    inline int sum_two(int a, int b) { return variadic_template_ns::sum_all(a, b); }
    inline int sum_five(int a, int b, int c, int d, int e) {
        return variadic_template_ns::sum_all(a, b, c, d, e);
    }
}

hicc::import_lib! {
    #![link_name = "variadic_template"]

    #[cpp(func = "std::string format_three(const std::string&, const std::string&, const std::string&)")]
    pub fn format_three(a: &hicc_std::string, b: &hicc_std::string, c: &hicc_std::string) -> hicc_std::string;

    #[cpp(func = "int variadic_template_ns::sum_three(int, int, int)")]
    pub fn sum_three(a: i32, b: i32, c: i32) -> i32;

    #[cpp(func = "int sum_two(int, int)")]
    pub fn sum_two(a: i32, b: i32) -> i32;

    #[cpp(func = "int sum_five(int, int, int, int, int)")]
    pub fn sum_five(a: i32, b: i32, c: i32, d: i32, e: i32) -> i32;

    class string;
}
