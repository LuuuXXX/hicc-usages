//! 自动生成：hicc_usage_constexpr_basic
//!
//! 由 tools/rust-gen/rust_gen.py 从 ../ast/symbols.json 生成。
//! 请勿手动编辑；如需调整请修改 tools/rust-gen/special.yaml 后重新生成。

hicc::cpp! {
    #include "hicc_usages/constexpr_basic.h"
}
hicc::import_class! {
    #[cpp(class = "hicc_usages::constexpr_basic::ConstContainer", destroy = "hicc_usages::constexpr_basic::ConstContainer::free")]
    pub class ConstContainer {
        #[cpp(method = "int get_factorial_5() const")]
        pub fn get_factorial_5(&self) -> i32;
        #[cpp(method = "int get_fibonacci_10() const")]
        pub fn get_fibonacci_10(&self) -> i32;
        #[cpp(method = "int get_power_2_10() const")]
        pub fn get_power_2_10(&self) -> i32;
        #[cpp(method = "int size() const")]
        pub fn size(&self) -> i32;
        #[cpp(method = "int at(int) const")]
        pub fn at(&self, idx: i32) -> i32;
    }
}
hicc::import_lib! {
    #![link_name = "hicc_usage_constexpr_basic_adapter"]
    pub class ConstContainer;
    #[cpp(func = "hicc_usages::constexpr_basic::ConstContainer * hicc_usages::constexpr_basic::ConstContainer::create()")]
    pub fn constcontainer_new() -> ConstContainer;
    #[cpp(func = "int hicc_usages::constexpr_basic::factorial(int)")]
    pub fn factorial(n: i32) -> i32;
    #[cpp(func = "int hicc_usages::constexpr_basic::fibonacci(int)")]
    pub fn fibonacci(n: i32) -> i32;
    #[cpp(func = "int hicc_usages::constexpr_basic::power(int, int)")]
    pub fn power(base: i32, exp: i32) -> i32;
    #[cpp(func = "int hicc_usages::constexpr_basic::call_factorial(int)")]
    pub fn call_factorial(n: i32) -> i32;
    #[cpp(func = "int hicc_usages::constexpr_basic::call_fibonacci(int)")]
    pub fn call_fibonacci(n: i32) -> i32;
    #[cpp(func = "int hicc_usages::constexpr_basic::call_power(int, int)")]
    pub fn call_power(base: i32, exp: i32) -> i32;
    #[cpp(func = "int hicc_usages::constexpr_basic::square_const(int)")]
    pub fn square_const(x: i32) -> i32;
}
