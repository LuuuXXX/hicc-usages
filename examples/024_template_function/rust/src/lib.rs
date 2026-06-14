//! 自动生成：hicc_usage_template_function
//!
//! 由 tools/rust-gen/rust_gen.py 从 ../ast/symbols.json 生成。
//! 请勿手动编辑；如需调整请修改 tools/rust-gen/special.yaml 后重新生成。

hicc::cpp! {
    #include "hicc_usages/template_function.h"
}
hicc::import_lib! {
    #![link_name = "hicc_usage_template_function_adapter"]
    #[cpp(func = "int hicc_usages::template_function::max_of<int>(int, int)")]
    pub fn max_of_int(a: i32, b: i32) -> i32;
    #[cpp(func = "double hicc_usages::template_function::max_of<double>(double, double)")]
    pub fn max_of_double(a: f64, b: f64) -> f64;
    #[cpp(func = "int hicc_usages::template_function::min_of<int>(int, int)")]
    pub fn min_of_int(a: i32, b: i32) -> i32;
    #[cpp(func = "double hicc_usages::template_function::min_of<double>(double, double)")]
    pub fn min_of_double(a: f64, b: f64) -> f64;
    #[cpp(func = "int hicc_usages::template_function::add_of<int>(int, int)")]
    pub fn add_of_int(a: i32, b: i32) -> i32;
    #[cpp(func = "double hicc_usages::template_function::add_of<double>(double, double)")]
    pub fn add_of_double(a: f64, b: f64) -> f64;
}
