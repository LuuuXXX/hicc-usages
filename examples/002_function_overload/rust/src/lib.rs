//! 自动生成：hicc_usage_function_overload
//!
//! 由 tools/rust-gen/rust_gen.py 从 ../ast/symbols.json 生成。
//! 请勿手动编辑；如需调整请修改 tools/rust-gen/special.yaml 后重新生成。

hicc::cpp! {
    #include "hicc_usages/function_overload.h"
}
hicc::import_lib! {
    #![link_name = "hicc_usage_function_overload_adapter"]
    #[cpp(func = "int hicc_usages::function_overload::add(int, int)")]
    pub fn add_int_int(a: i32, b: i32) -> i32;
    #[cpp(func = "int hicc_usages::function_overload::add(int, int, int)")]
    pub fn add_int_int_int(a: i32, b: i32, c: i32) -> i32;
}
