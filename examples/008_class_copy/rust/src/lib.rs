//! 自动生成：hicc_usage_class_copy
//!
//! 由 tools/rust-gen/rust_gen.py 从 ../ast/symbols.json 生成。
//! 请勿手动编辑；如需调整请修改 tools/rust-gen/special.yaml 后重新生成。

hicc::cpp! {
    #include "hicc_usages/class_copy.h"
}
hicc::import_class! {
    #[cpp(class = "hicc_usages::class_copy::Buffer", destroy = "hicc_usages::class_copy::Buffer::free")]
    pub class Buffer {
        #[cpp(method = "int capacity() const")]
        pub fn capacity(&self) -> i32;
        #[cpp(method = "int size() const")]
        pub fn size(&self) -> i32;
        #[cpp(method = "void append(int)")]
        pub fn append(&mut self, value: i32) -> ();
    }
}
hicc::import_lib! {
    #![link_name = "hicc_usage_class_copy_adapter"]
    pub class Buffer;
    #[cpp(func = "hicc_usages::class_copy::Buffer * hicc_usages::class_copy::Buffer::create(int)")]
    pub fn buffer_new(capacity: i32) -> Buffer;
}
