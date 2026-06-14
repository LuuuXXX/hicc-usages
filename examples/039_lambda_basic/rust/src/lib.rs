//! 自动生成：hicc_usage_lambda_basic
//!
//! 由 tools/rust-gen/rust_gen.py 从 ../ast/symbols.json 生成。
//! 请勿手动编辑；如需调整请修改 tools/rust-gen/special.yaml 后重新生成。

hicc::cpp! {
    #include "hicc_usages/lambda_basic.h"
}
hicc::import_class! {
    #[cpp(class = "hicc_usages::lambda_basic::Calculator", destroy = "hicc_usages::lambda_basic::Calculator::free")]
    pub class Calculator {
        #[cpp(method = "int add(int, int) const")]
        pub fn add(&self, a: i32, b: i32) -> i32;
        #[cpp(method = "int multiply(int, int) const")]
        pub fn multiply(&self, a: i32, b: i32) -> i32;
        #[cpp(method = "int square(int) const")]
        pub fn square(&self, x: i32) -> i32;
        #[cpp(method = "int factorial(int) const")]
        pub fn factorial(&self, n: i32) -> i32;
    }
}
hicc::import_lib! {
    #![link_name = "hicc_usage_lambda_basic_adapter"]
    pub class Calculator;
    #[cpp(func = "hicc_usages::lambda_basic::Calculator * hicc_usages::lambda_basic::Calculator::create()")]
    pub fn calculator_new() -> Calculator;
    #[cpp(func = "int hicc_usages::lambda_basic::apply_double(int)")]
    pub fn apply_double(x: i32) -> i32;
    #[cpp(func = "int hicc_usages::lambda_basic::apply_square(int)")]
    pub fn apply_square(x: i32) -> i32;
    #[cpp(func = "int hicc_usages::lambda_basic::apply_negate(int)")]
    pub fn apply_negate(x: i32) -> i32;
    #[cpp(func = "int hicc_usages::lambda_basic::sum_with_lambda(int, int, int)")]
    pub fn sum_with_lambda(a: i32, b: i32, c: i32) -> i32;
    #[cpp(func = "int hicc_usages::lambda_basic::count_if_positive(int *, std::size_t)")]
    pub fn count_if_positive(values: *mut i32, count: usize) -> i32;
}
