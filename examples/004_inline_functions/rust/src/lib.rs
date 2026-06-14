//! 自动生成：hicc_usage_inline_functions
//!
//! 由 tools/rust-gen/rust_gen.py 从 ../ast/symbols.json 生成。
//! 请勿手动编辑；如需调整请修改 tools/rust-gen/special.yaml 后重新生成。

hicc::cpp! {
    #include "hicc_usages/inline_functions.h"
}
hicc::import_lib! {
    #![link_name = "hicc_usage_inline_functions_adapter"]
    #[cpp(func = "int hicc_usages::inline_functions::square(int)")]
    pub fn square(x: i32) -> i32;
    #[cpp(func = "int hicc_usages::inline_functions::cube(int)")]
    pub fn cube(x: i32) -> i32;
    #[cpp(func = "int hicc_usages::inline_functions::compute(int)")]
    pub fn compute(x: i32) -> i32;
}
