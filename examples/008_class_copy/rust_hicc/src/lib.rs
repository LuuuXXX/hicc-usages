//! 008_class_copy: copy/move 语义 + 工厂。
//!
//! hicc 模式：
//! - 构造工厂调用对应 ctor
//! - Rust 端通过 `clone`（调 copy ctor 工厂）和 `move_from`（调 move ctor 工厂）模拟语义
//! - operator=（赋值）用 `AbiClass::write()` 实现

hicc::cpp! {
    #include "class_copy.h"
    #include <hicc/std/string.hpp>
}

hicc::import_class! {
    class string = hicc_std::string;

    #[cpp(class = "class_copy_ns::Buffer")]
    pub class Buffer {
        #[cpp(method = "int size() const")]
        pub fn size(&self) -> i32;

        #[cpp(method = "const std::string& tag() const")]
        pub fn tag(&self) -> &string;

        pub fn new(sz: i32, tag: &string) -> Self {
            buffer_new(sz, tag)
        }
        pub fn default_() -> Self {
            buffer_default()
        }
        pub fn clone(&self) -> Self {
            buffer_clone(self)
        }
        pub fn move_from(other: Self) -> Self {
            buffer_move(other)
        }
    }
}

hicc::import_lib! {
    #![link_name = "class_copy"]

    #[cpp(func = "std::unique_ptr<class_copy_ns::Buffer> hicc::make_unique<class_copy_ns::Buffer, int, const std::string&>(int&&, const std::string&)")]
    pub fn buffer_new(sz: i32, tag: &hicc_std::string) -> Buffer;

    #[cpp(func = "std::unique_ptr<class_copy_ns::Buffer> hicc::make_unique<class_copy_ns::Buffer>()")]
    pub fn buffer_default() -> Buffer;

    // copy ctor: 接收 const Buffer&
    #[cpp(func = "std::unique_ptr<class_copy_ns::Buffer> hicc::make_unique<class_copy_ns::Buffer, const class_copy_ns::Buffer&>(const class_copy_ns::Buffer&)")]
    pub fn buffer_clone(other: &Buffer) -> Buffer;

    // move ctor: 接收 Buffer&&，对应 Rust 端的 Self（按值传递）
    #[cpp(func = "std::unique_ptr<class_copy_ns::Buffer> hicc::make_unique<class_copy_ns::Buffer, class_copy_ns::Buffer&&>(class_copy_ns::Buffer&&)")]
    pub fn buffer_move(other: Buffer) -> Buffer;
}
