//! 自动生成：hicc_usage_virtual_override
//!
//! 由 tools/rust-gen/rust_gen.py 从 ../ast/symbols.json 生成。
//! 请勿手动编辑；如需调整请修改 tools/rust-gen/special.yaml 后重新生成。

hicc::cpp! {
    #include "hicc_usages/virtual_override.h"
}
hicc::import_class! {
    #[cpp(class = "hicc_usages::virtual_override::Base", destroy = "hicc_usages::virtual_override::Base::free")]
    pub class Base {
        #[cpp(method = "const char * name() const")]
        pub fn name(&self) -> *const i8;
        #[cpp(method = "int compute(int) const")]
        pub fn compute(&self, x: i32) -> i32;
    }
}
hicc::import_class! {
    #[cpp(class = "hicc_usages::virtual_override::Derived", destroy = "hicc_usages::virtual_override::Derived::free")]
    pub class Derived {
        #[cpp(method = "const char * name() const")]
        pub fn name(&self) -> *const i8;
        #[cpp(method = "int compute(int) const")]
        pub fn compute(&self, x: i32) -> i32;
        #[cpp(method = "int multiplier() const")]
        pub fn multiplier(&self) -> i32;
    }
}
hicc::import_lib! {
    #![link_name = "hicc_usage_virtual_override_adapter"]
    pub class Base;
    pub class Derived;
    #[cpp(func = "hicc_usages::virtual_override::Base * hicc_usages::virtual_override::Base::create()")]
    pub fn base_new() -> Base;
    #[cpp(func = "hicc_usages::virtual_override::Derived * hicc_usages::virtual_override::Derived::create(int)")]
    pub fn derived_new(multiplier: i32) -> Derived;
}
