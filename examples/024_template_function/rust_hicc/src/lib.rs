//! 024_template_function: 模板函数
//!
//! hicc 模式：模板函数不能直接绑（符号不固定）。C++ 端 `template ... add<int>(...)`
//! 显式实例化后，**cpp! 块写 inline 包装**把这些函数转成 by-value 参数 / by-value 返回，
//! 再 import_lib! 绑包装函数。原因：hicc 不直接接 `const T&` 对原始类型（int/double）。

hicc::cpp! {
    #include "template_function.h"
    #include <hicc/std/string.hpp>

    inline int add_int_wrap(int a, int b) {
        return template_function_ns::add<int>(a, b);
    }
    inline double add_double_wrap(double a, double b) {
        return template_function_ns::add<double>(a, b);
    }
    inline int max_of_int_wrap(int a, int b) {
        return template_function_ns::max_of<int>(a, b);
    }
    inline std::string describe_int_wrap(int v) {
        return template_function_ns::describe<int>(v);
    }
}

hicc::import_lib! {
    #![link_name = "template_function"]

    #[cpp(func = "int add_int_wrap(int, int)")]
    pub fn add_int(a: i32, b: i32) -> i32;

    #[cpp(func = "double add_double_wrap(double, double)")]
    pub fn add_double(a: f64, b: f64) -> f64;

    #[cpp(func = "int max_of_int_wrap(int, int)")]
    pub fn max_of_int(a: i32, b: i32) -> i32;

    #[cpp(func = "std::string describe_int_wrap(int)")]
    pub fn describe_int(v: i32) -> hicc_std::string;

    class string;
}
