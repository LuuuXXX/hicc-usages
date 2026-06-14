//! 自动生成：hicc_usage_class_basic
//!
//! 由 tools/rust-gen/rust_gen.py 从 ../ast/symbols.json 生成。
//! 请勿手动编辑；如需调整请修改 tools/rust-gen/special.yaml 后重新生成。

hicc::cpp! {
    #include "hicc_usages/class_basic.h"
}
hicc::import_class! {
    #[cpp(class = "hicc_usages::class_basic::Counter", destroy = "hicc_usages::class_basic::Counter::free")]
    pub class Counter {
        #[cpp(method = "int get() const")]
        pub fn get(&self) -> i32;
        #[cpp(method = "void increment()")]
        pub fn increment(&mut self) -> ();
        #[cpp(method = "void decrement()")]
        pub fn decrement(&mut self) -> ();
        #[cpp(method = "void reset()")]
        pub fn reset(&mut self) -> ();
    }
}
hicc::import_lib! {
    #![link_name = "hicc_usage_class_basic_adapter"]
    pub class Counter;
    #[cpp(func = "hicc_usages::class_basic::Counter * hicc_usages::class_basic::Counter::create()")]
    pub fn counter_new() -> Counter;
}
