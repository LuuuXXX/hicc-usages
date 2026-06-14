//! 012_class_volatile: volatile 成员函数与字段
//!
//! hicc 模式：volatile 在 Rust FFI 中无对应语义，调用 C++ 的 volatile 方法
//! 会因 vtable/this 调用 ABI 不匹配而出错。C++ 端提供非 volatile 桥接
//! (safe_read/safe_write)，Rust 只绑定桥接方法。

hicc::cpp! {
    #include "class_volatile.h"
    #include <hicc/std/string.hpp>
}

hicc::import_class! {
    #[cpp(class = "class_volatile_ns::Sensor")]
    pub class Sensor {
        #[cpp(method = "int id() const")]
        pub fn id(&self) -> i32;

        #[cpp(method = "int counter() const")]
        pub fn counter(&self) -> i32;

        #[cpp(method = "int safe_read() const")]
        pub fn safe_read(&self) -> i32;

        #[cpp(method = "void safe_write(int)")]
        pub fn safe_write(&mut self, v: i32);

        pub fn new(id: i32) -> Self { sensor_new(id) }
    }
}

hicc::import_lib! {
    #![link_name = "class_volatile"]

    #[cpp(func = "std::unique_ptr<class_volatile_ns::Sensor> hicc::make_unique<class_volatile_ns::Sensor, int>(int&&)")]
    pub fn sensor_new(id: i32) -> Sensor;
}
