//! 自动生成：hicc_usage_default_args
//!
//! 由 tools/rust-gen/rust_gen.py 从 ../ast/symbols.json 生成。
//! 请勿手动编辑；如需调整请修改 tools/rust-gen/special.yaml 后重新生成。

hicc::cpp! {
    #include "hicc_usages/default_args.h"
}
hicc::import_lib! {
    #![link_name = "hicc_usage_default_args_adapter"]
    #[cpp(func = "int hicc_usages::default_args::power(int, int)")]
    pub fn power_int_int(base: i32, exp: i32) -> i32;
    #[cpp(func = "int hicc_usages::default_args::power(int)")]
    pub fn power_int(base: i32) -> i32;
    #[cpp(func = "int hicc_usages::default_args::power()")]
    pub fn power_v() -> i32;
}
