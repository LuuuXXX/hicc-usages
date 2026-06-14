//! 自动生成：hicc_usage_class_volatile
//!
//! 由 tools/rust-gen/rust_gen.py 从 ../ast/symbols.json 生成。
//! 请勿手动编辑；如需调整请修改 tools/rust-gen/special.yaml 后重新生成。

hicc::cpp! {
    #include "hicc_usages/class_volatile.h"
}
hicc::import_class! {
    #[cpp(class = "hicc_usages::class_volatile::Sensor", destroy = "hicc_usages::class_volatile::Sensor::free")]
    pub class Sensor {
        #[cpp(method = "int read() const")]
        pub fn read(&self) -> i32;
        #[cpp(method = "void update(int)")]
        pub fn update(&mut self, v: i32) -> ();
        #[cpp(method = "int read_volatile() const volatile")]
        pub fn read_volatile(&self) -> i32;
    }
}
hicc::import_lib! {
    #![link_name = "hicc_usage_class_volatile_adapter"]
    pub class Sensor;
    #[cpp(func = "hicc_usages::class_volatile::Sensor * hicc_usages::class_volatile::Sensor::create()")]
    pub fn sensor_new() -> Sensor;
}
