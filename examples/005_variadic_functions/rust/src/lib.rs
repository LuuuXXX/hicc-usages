//! 自动生成：hicc_usage_variadic_functions
//!
//! 由 tools/rust-gen/rust_gen.py 从 ../ast/symbols.json 生成。
//! 请勿手动编辑；如需调整请修改 tools/rust-gen/special.yaml 后重新生成。

hicc::cpp! {
    #include "hicc_usages/variadic_functions.h"
}
hicc::import_lib! {
    #![link_name = "hicc_usage_variadic_functions_adapter"]
    #[cpp(func = "int hicc_usages::variadic_functions::sum_2(int, int)")]
    pub fn sum_2(a: i32, b: i32) -> i32;
    #[cpp(func = "int hicc_usages::variadic_functions::sum_3(int, int, int)")]
    pub fn sum_3(a: i32, b: i32, c: i32) -> i32;
    #[cpp(func = "int hicc_usages::variadic_functions::sum_4(int, int, int, int)")]
    pub fn sum_4(a: i32, b: i32, c: i32, d: i32) -> i32;
    #[cpp(func = "int hicc_usages::variadic_functions::sum_array(const int *, int)")]
    pub fn sum_array(arr: *const i32, count: i32) -> i32;
}
