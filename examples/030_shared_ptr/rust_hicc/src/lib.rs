//! 030_shared_ptr: std::shared_ptr
//!
//! hicc 模式：shared_ptr 工厂返回 `hicc::shared_ptr<T>`（不是 T 本身）。
//! 引用计数、clone、weak_ptr 都通过 hicc::shared_ptr API。
//! 注意：`shared_ptr<T>` 参数/返回在 hicc 中映射到 `hicc::shared_ptr<T>` Rust 类型，
//! 不能直接当作 T 使用（与 unique_ptr 默认 deleter 不同）。

hicc::cpp! {
    #include "shared_ptr.h"
    #include <hicc/std/memory.hpp>
}

hicc::import_class! {
    #[cpp(class = "shared_ptr_ns::Counter")]
    pub class Counter {
        #[cpp(method = "int value() const")]
        pub fn value(&self) -> i32;

        #[cpp(method = "void increment()")]
        pub fn increment(&mut self);

        #[cpp(method = "void decrement()")]
        pub fn decrement(&mut self);
    }
}

hicc::import_lib! {
    #![link_name = "shared_ptr"]

    #[cpp(func = "std::shared_ptr<shared_ptr_ns::Counter> shared_ptr_ns::make_counter(int)")]
    pub fn make_counter(start: i32) -> hicc::shared_ptr<Counter>;

    #[cpp(func = "std::shared_ptr<shared_ptr_ns::Counter> shared_ptr_ns::clone_counter(const std::shared_ptr<shared_ptr_ns::Counter>&)")]
    pub fn clone_counter(other: &hicc::shared_ptr<Counter>) -> hicc::shared_ptr<Counter>;

    #[cpp(func = "long shared_ptr_ns::use_count(const std::shared_ptr<shared_ptr_ns::Counter>&)")]
    pub fn use_count(p: &hicc::shared_ptr<Counter>) -> i64;
}
