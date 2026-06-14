//! 029_unique_ptr: std::unique_ptr 默认 deleter
//!
//! hicc 模式：unique_ptr 默认 deleter 时，hicc 把工厂返回值映射为类对象本身（不是
//! `hicc::unique_ptr<T>`）。`std::unique_ptr<T>` 参数（消费）同样按 T 接收。
//! 工厂用 `hicc::make_unique<T, Args&&...>(Args&&...)` 返回类对象本身。
//! **Rust 端 Resource 的 Drop 等价于 C++ 端 unique_ptr 的释放**，
//! 不需要再绑 `consume_resource(std::unique_ptr<T>)` — 该参数触发了 hicc 已知 bug
//! (`make_unique_arg` 未定义)。为了演示 consume 语义，用 Rust Drop 演示。

hicc::cpp! {
    #include "unique_ptr.h"
    #include <hicc/std/string.hpp>

    // 直接复用 hicc::make_unique 而不调 unique_ptr_ns::make_resource（也返回 unique_ptr）
    // consume 语义用 Rust DROP 演示
}

hicc::import_class! {
    class string = hicc_std::string;

    #[cpp(class = "unique_ptr_ns::Resource")]
    pub class Resource {
        #[cpp(method = "int id() const")]
        pub fn id(&self) -> i32;

        #[cpp(method = "const std::string& name() const")]
        pub fn name(&self) -> &string;

        pub fn new(id: i32, name: &string) -> Self { make_resource(id, name) }
    }
}

hicc::import_lib! {
    #![link_name = "unique_ptr"]

    #[cpp(func = "std::unique_ptr<unique_ptr_ns::Resource> hicc::make_unique<unique_ptr_ns::Resource, int, const std::string&>(int&&, const std::string&)")]
    pub fn make_resource(id: i32, name: &hicc_std::string) -> Resource;
}
