//! 自动生成：hicc_usage_class_static
//!
//! 由 tools/rust-gen/rust_gen.py 从 ../ast/symbols.json 生成。
//! 请勿手动编辑；如需调整请修改 tools/rust-gen/special.yaml 后重新生成。

hicc::cpp! {
    #include "hicc_usages/class_static.h"
}
hicc::import_class! {
    #[cpp(class = "hicc_usages::class_static::Counter", destroy = "hicc_usages::class_static::Counter::free")]
    pub class Counter {
        #[cpp(method = "void tick()")]
        pub fn tick(&mut self) -> ();
        #[cpp(method = "int get_ticks() const")]
        pub fn get_ticks(&self) -> i32;
    }
}
hicc::import_lib! {
    #![link_name = "hicc_usage_class_static_adapter"]
    pub class Counter;
    #[cpp(func = "hicc_usages::class_static::Counter * hicc_usages::class_static::Counter::create()")]
    pub fn counter_new() -> Counter;
    #[cpp(func = "int hicc_usages::class_static::Counter::get_instance_count()")]
    pub fn counter_get_instance_count() -> i32;
}
