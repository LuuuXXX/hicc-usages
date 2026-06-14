//! 035_map_basic: std::map
//!
//! hicc 模式：`hicc_std::map<K, V>` 别名 + `make_unique` 工厂。
//! K 为 Pod<i32>，V 为 hicc_std::string。
//! Rust 端使用 hicc_std::map 的内置方法（insert / get / size 等）。

hicc::cpp! {
    #include "map_basic.h"
    #include <hicc/std/map.hpp>
    #include <hicc/std/string.hpp>
    typedef std::map<int, std::string> CppMap;
}

hicc::import_lib! {
    #![link_name = "map_basic"]

    class RustMap = hicc_std::map<hicc::Pod<i32>, hicc_std::string>;

    #[cpp(func = "std::unique_ptr<CppMap> hicc::make_unique<CppMap>()")]
    pub fn map_new() -> RustMap;

    #[cpp(func = "void map_basic_ns::put(std::map<int, std::string>&, int, const std::string&)")]
    pub fn put(m: &mut RustMap, key: i32, val: &hicc_std::string);

    #[cpp(func = "std::string map_basic_ns::get_or(const std::map<int, std::string>&, int, const std::string&)")]
    pub fn get_or(m: &RustMap, key: i32, def: &hicc_std::string) -> hicc_std::string;

    #[cpp(func = "size_t map_basic_ns::map_size(const std::map<int, std::string>&)")]
    pub fn map_size(m: &RustMap) -> usize;

    #[cpp(func = "long map_basic_ns::sum_key_values(const std::map<int, std::string>&)")]
    pub fn sum_key_values(m: &RustMap) -> i64;
}
