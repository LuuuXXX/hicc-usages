//! 自动生成：hicc_usage_map_basic
//!
//! 由 tools/rust-gen/rust_gen.py 从 ../ast/symbols.json 生成。
//! 请勿手动编辑；如需调整请修改 tools/rust-gen/special.yaml 后重新生成。

hicc::cpp! {
    #include "hicc_usages/map_basic.h"
}
hicc::import_class! {
    #[cpp(class = "hicc_usages::map_basic::IntMap", destroy = "hicc_usages::map_basic::IntMap::free")]
    pub class IntMap {
        #[cpp(method = "void put(int, int)")]
        pub fn put(&mut self, key: i32, value: i32) -> ();
        #[cpp(method = "bool has(int) const")]
        pub fn has(&self, key: i32) -> bool;
        #[cpp(method = "int get(int) const")]
        pub fn get(&self, key: i32) -> i32;
        #[cpp(method = "void erase(int)")]
        pub fn erase(&mut self, key: i32) -> ();
        #[cpp(method = "int sum_values() const")]
        pub fn sum_values(&self) -> i32;
    }
}
hicc::import_lib! {
    #![link_name = "hicc_usage_map_basic_adapter"]
    pub class IntMap;
    #[cpp(func = "hicc_usages::map_basic::IntMap * hicc_usages::map_basic::IntMap::create()")]
    pub fn intmap_new() -> IntMap;
}
