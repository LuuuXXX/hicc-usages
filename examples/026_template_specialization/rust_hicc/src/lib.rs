//! 026_template_specialization: 模板偏特化
//!
//! hicc 模式：每个偏特化版当独立类。但 C++ 端是 struct + 静态方法，
//! hicc 无法直接通过 #[cpp(method = "...")] 访问 static — 改用 cpp! 包装。

hicc::cpp! {
    #include "template_specialization.h"
    #include <hicc/std/string.hpp>

    inline const char* typeinfo_int_name() { return template_specialization_ns::TypeInfo<int>::name(); }
    inline std::string typeinfo_int_describe(int v) { return template_specialization_ns::TypeInfo<int>::describe(v); }
    inline const char* typeinfo_double_name() { return template_specialization_ns::TypeInfo<double>::name(); }
    inline std::string typeinfo_double_describe(double v) { return template_specialization_ns::TypeInfo<double>::describe(v); }
    inline const char* typeinfo_string_name() { return template_specialization_ns::TypeInfo<std::string>::name(); }
    inline std::string typeinfo_string_describe(const std::string& v) { return template_specialization_ns::TypeInfo<std::string>::describe(v); }
}

hicc::import_lib! {
    #![link_name = "template_specialization"]

    #[cpp(func = "const char* typeinfo_int_name()")]
    pub fn int_name() -> *const i8;

    #[cpp(func = "std::string typeinfo_int_describe(int)")]
    pub fn int_describe(v: i32) -> hicc_std::string;

    #[cpp(func = "const char* typeinfo_double_name()")]
    pub fn double_name() -> *const i8;

    #[cpp(func = "std::string typeinfo_double_describe(double)")]
    pub fn double_describe(v: f64) -> hicc_std::string;

    #[cpp(func = "const char* typeinfo_string_name()")]
    pub fn string_name() -> *const i8;

    #[cpp(func = "std::string typeinfo_string_describe(const std::string&)")]
    pub fn string_describe(v: &hicc_std::string) -> hicc_std::string;

    class string;
}
