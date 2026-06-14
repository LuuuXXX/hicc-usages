//! 自动生成：hicc_usage_class_const
//!
//! 由 tools/rust-gen/rust_gen.py 从 ../ast/symbols.json 生成。
//! 请勿手动编辑；如需调整请修改 tools/rust-gen/special.yaml 后重新生成。

hicc::cpp! {
    #include "hicc_usages/class_const.h"
}
hicc::import_class! {
    #[cpp(class = "hicc_usages::class_const::Value", destroy = "hicc_usages::class_const::Value::free")]
    pub class Value {
        #[cpp(method = "int get() const")]
        pub fn get(&self) -> i32;
        #[cpp(method = "void set(int)")]
        pub fn set(&mut self, v: i32) -> ();
        #[cpp(method = "void add(int)")]
        pub fn add(&mut self, delta: i32) -> ();
    }
}
hicc::import_lib! {
    #![link_name = "hicc_usage_class_const_adapter"]
    pub class Value;
    #[cpp(func = "hicc_usages::class_const::Value * hicc_usages::class_const::Value::create(int)")]
    pub fn value_new(initial: i32) -> Value;
}
