//! 016_virtual_pure: 纯虚函数（= 0）+ 抽象基类
//!
//! hicc 模式：抽象基类 Storage 无法实例化，只绑定具体派生类 InMemoryStorage。
//! 纯虚方法在派生类已实现，可直接通过 #[cpp(method = "...")] 暴露。

hicc::cpp! {
    #include "virtual_pure.h"
    #include <hicc/std/string.hpp>
}

hicc::import_class! {
    class string = hicc_std::string;

    #[cpp(class = "virtual_pure_ns::InMemoryStorage")]
    pub class InMemoryStorage {
        #[cpp(method = "bool put(const std::string&, const std::string&)")]
        pub fn put(&mut self, key: &string, value: &string) -> bool;

        #[cpp(method = "std::string get(const std::string&) const")]
        pub fn get(&self, key: &string) -> string;

        #[cpp(method = "bool remove(const std::string&)")]
        pub fn remove(&mut self, key: &string) -> bool;

        #[cpp(method = "size_t size() const")]
        pub fn size(&self) -> usize;

        #[cpp(method = "void dump() const")]
        pub fn dump(&self);

        pub fn new() -> Self { in_memory_storage_new() }
    }
}

hicc::import_lib! {
    #![link_name = "virtual_pure"]

    #[cpp(func = "std::unique_ptr<virtual_pure_ns::InMemoryStorage> hicc::make_unique<virtual_pure_ns::InMemoryStorage>()")]
    pub fn in_memory_storage_new() -> InMemoryStorage;
}
