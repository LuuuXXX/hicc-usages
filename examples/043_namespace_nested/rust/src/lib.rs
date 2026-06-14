//! 自动生成：hicc_usage_namespace_nested
//!
//! 由 tools/rust-gen/rust_gen.py 从 ../ast/symbols.json 生成。
//! 请勿手动编辑；如需调整请修改 tools/rust-gen/special.yaml 后重新生成。

hicc::cpp! {
    #include "hicc_usages/namespace_nested.h"
}
hicc::import_class! {
    #[cpp(class = "hicc_usages::namespace_nested::outer::inner::Calculator", destroy = "hicc_usages::namespace_nested::outer::inner::Calculator::free")]
    pub class Calculator {
        #[cpp(method = "int compute(int, int) const")]
        pub fn compute(&self, a: i32, b: i32) -> i32;
    }
}
hicc::import_class! {
    #[cpp(class = "hicc_usages::namespace_nested::outer::Helper", destroy = "hicc_usages::namespace_nested::outer::Helper::free")]
    pub class Helper {
        #[cpp(method = "int doubled(int) const")]
        pub fn doubled(&self, x: i32) -> i32;
    }
}
hicc::import_lib! {
    #![link_name = "hicc_usage_namespace_nested_adapter"]
    pub class Calculator;
    pub class Helper;
    #[cpp(func = "hicc_usages::namespace_nested::outer::inner::Calculator * hicc_usages::namespace_nested::outer::inner::Calculator::create()")]
    pub fn calculator_new() -> Calculator;
    #[cpp(func = "hicc_usages::namespace_nested::outer::Helper * hicc_usages::namespace_nested::outer::Helper::create()")]
    pub fn helper_new() -> Helper;
    #[cpp(func = "int hicc_usages::namespace_nested::outer::inner::add(int, int)")]
    pub fn add(a: i32, b: i32) -> i32;
    #[cpp(func = "int hicc_usages::namespace_nested::outer::inner::multiply(int, int)")]
    pub fn multiply(a: i32, b: i32) -> i32;
    #[cpp(func = "int hicc_usages::namespace_nested::outer::subtract(int, int)")]
    pub fn subtract(a: i32, b: i32) -> i32;
    #[cpp(func = "int hicc_usages::namespace_nested::outer_inner_sum(int, int, int)")]
    pub fn outer_inner_sum(a: i32, b: i32, c: i32) -> i32;
}
