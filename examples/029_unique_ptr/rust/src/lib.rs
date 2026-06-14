//! 自动生成：hicc_usage_unique_ptr
//!
//! 由 tools/rust-gen/rust_gen.py 从 ../ast/symbols.json 生成。
//! 请勿手动编辑；如需调整请修改 tools/rust-gen/special.yaml 后重新生成。

hicc::cpp! {
    #include "hicc_usages/unique_ptr.h"
}
hicc::import_class! {
    #[cpp(class = "hicc_usages::unique_ptr::Resource", destroy = "hicc_usages::unique_ptr::Resource::free")]
    pub class Resource {
        #[cpp(method = "int id() const")]
        pub fn id(&self) -> i32;
        #[cpp(method = "void touch()")]
        pub fn touch(&mut self) -> ();
        #[cpp(method = "int touches() const")]
        pub fn touches(&self) -> i32;
    }
}
hicc::import_class! {
    #[cpp(class = "hicc_usages::unique_ptr::Owner", destroy = "hicc_usages::unique_ptr::Owner::free")]
    pub class Owner {
        #[cpp(method = "void release(int)")]
        pub fn release(&mut self, id: i32) -> ();
        #[cpp(method = "bool has(int) const")]
        pub fn has(&self, id: i32) -> bool;
    }
}
hicc::import_lib! {
    #![link_name = "hicc_usage_unique_ptr_adapter"]
    pub class Resource;
    pub class Owner;
    #[cpp(func = "hicc_usages::unique_ptr::Resource * hicc_usages::unique_ptr::Resource::create(int)")]
    pub fn resource_new(id: i32) -> Resource;
    #[cpp(func = "hicc_usages::unique_ptr::Owner * hicc_usages::unique_ptr::Owner::create()")]
    pub fn owner_new() -> Owner;
}
