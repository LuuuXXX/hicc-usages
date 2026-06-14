//! 自动生成：hicc_usage_mutable_member
//!
//! 由 tools/rust-gen/rust_gen.py 从 ../ast/symbols.json 生成。
//! 请勿手动编辑；如需调整请修改 tools/rust-gen/special.yaml 后重新生成。

hicc::cpp! {
    #include "hicc_usages/mutable_member.h"
}
hicc::import_class! {
    #[cpp(class = "hicc_usages::mutable_member::Cache", destroy = "hicc_usages::mutable_member::Cache::free")]
    pub class Cache {
        #[cpp(method = "int get_value(int) const")]
        pub fn get_value(&self, key: i32) -> i32;
        #[cpp(method = "void set_value(int, int)")]
        pub fn set_value(&mut self, key: i32, value: i32) -> ();
        #[cpp(method = "int access_count() const")]
        pub fn access_count(&self) -> i32;
    }
}
hicc::import_lib! {
    #![link_name = "hicc_usage_mutable_member_adapter"]
    pub class Cache;
    #[cpp(func = "hicc_usages::mutable_member::Cache * hicc_usages::mutable_member::Cache::create()")]
    pub fn cache_new() -> Cache;
}
