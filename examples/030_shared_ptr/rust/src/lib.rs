//! 自动生成：hicc_usage_shared_ptr
//!
//! 由 tools/rust-gen/rust_gen.py 从 ../ast/symbols.json 生成。
//! 请勿手动编辑；如需调整请修改 tools/rust-gen/special.yaml 后重新生成。

hicc::cpp! {
    #include "hicc_usages/shared_ptr.h"
}
hicc::import_class! {
    #[cpp(class = "hicc_usages::shared_ptr::Counter", destroy = "hicc_usages::shared_ptr::Counter::free")]
    pub class Counter {
        #[cpp(method = "int value() const")]
        pub fn value(&self) -> i32;
        #[cpp(method = "void increment()")]
        pub fn increment(&mut self) -> ();
        #[cpp(method = "void decrement()")]
        pub fn decrement(&mut self) -> ();
    }
}
hicc::import_class! {
    #[cpp(class = "hicc_usages::shared_ptr::Registry", destroy = "hicc_usages::shared_ptr::Registry::free")]
    pub class Registry {
        #[cpp(method = "int sum() const")]
        pub fn sum(&self) -> i32;
    }
}
hicc::import_lib! {
    #![link_name = "hicc_usage_shared_ptr_adapter"]
    pub class Counter;
    pub class Registry;
    #[cpp(func = "hicc_usages::shared_ptr::Counter * hicc_usages::shared_ptr::Counter::create(int)")]
    pub fn counter_new(initial: i32) -> Counter;
    #[cpp(func = "hicc_usages::shared_ptr::Registry * hicc_usages::shared_ptr::Registry::create()")]
    pub fn registry_new() -> Registry;
}
