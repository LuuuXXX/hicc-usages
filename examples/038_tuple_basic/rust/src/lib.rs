//! 自动生成：hicc_usage_tuple_basic
//!
//! 由 tools/rust-gen/rust_gen.py 从 ../ast/symbols.json 生成。
//! 请勿手动编辑；如需调整请修改 tools/rust-gen/special.yaml 后重新生成。

hicc::cpp! {
    #include "hicc_usages/tuple_basic.h"
}
hicc::import_class! {
    #[cpp(class = "hicc_usages::tuple_basic::Triple", destroy = "hicc_usages::tuple_basic::Triple::free")]
    pub class Triple {
        #[cpp(method = "int first() const")]
        pub fn first(&self) -> i32;
        #[cpp(method = "double second() const")]
        pub fn second(&self) -> f64;
        #[cpp(method = "int third() const")]
        pub fn third(&self) -> i32;
        #[cpp(method = "void set_first(int)")]
        pub fn set_first(&mut self, v: i32) -> ();
        #[cpp(method = "void set_second(double)")]
        pub fn set_second(&mut self, v: f64) -> ();
        #[cpp(method = "void set_third(int)")]
        pub fn set_third(&mut self, v: i32) -> ();
        #[cpp(method = "int sum_ints() const")]
        pub fn sum_ints(&self) -> i32;
    }
}
hicc::import_lib! {
    #![link_name = "hicc_usage_tuple_basic_adapter"]
    pub class Triple;
    #[cpp(func = "hicc_usages::tuple_basic::Triple * hicc_usages::tuple_basic::Triple::create(int, double, int)")]
    pub fn triple_new(a: i32, b: f64, c: i32) -> Triple;
}
