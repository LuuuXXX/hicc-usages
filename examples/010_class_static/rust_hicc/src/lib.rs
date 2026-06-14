//! 010_class_static: 静态成员函数/变量
//!
//! hicc 模式：静态方法/字段用 cpp! 块的命名空间级包装函数 + import_lib! 导出
//! 实例方法（inc/count/id）走 import_class! 的 method 路径。

hicc::cpp! {
    #include "class_static.h"
    #include <hicc/std/string.hpp>

    inline int class_static_alive() { return class_static_ns::Counter::alive(); }
    inline int class_static_next_id() { return class_static_ns::Counter::next_id(); }
    inline const std::string& class_static_species() { return class_static_ns::Counter::species(); }
    inline int class_static_total_created() { return class_static_ns::Counter::s_total_created; }
    inline void class_static_add_total(int n) { class_static_ns::Counter::s_total_created += n; }
}

hicc::import_class! {
    class string = hicc_std::string;

    #[cpp(class = "class_static_ns::Counter")]
    pub class Counter {
        #[cpp(method = "int count() const")]
        pub fn count(&self) -> i32;

        #[cpp(method = "int id() const")]
        pub fn id(&self) -> i32;

        #[cpp(method = "void inc()")]
        pub fn inc(&mut self);

        pub fn new() -> Self { counter_new() }
    }
}

hicc::import_lib! {
    #![link_name = "class_static"]

    #[cpp(func = "std::unique_ptr<class_static_ns::Counter> hicc::make_unique<class_static_ns::Counter>()")]
    pub fn counter_new() -> Counter;

    #[cpp(func = "int class_static_alive()")]
    pub fn alive() -> i32;

    #[cpp(func = "int class_static_next_id()")]
    pub fn next_id() -> i32;

    #[cpp(func = "const std::string& class_static_species()")]
    pub fn species() -> string;

    #[cpp(func = "int class_static_total_created()")]
    pub fn total_created() -> i32;

    #[cpp(func = "void class_static_add_total(int)")]
    pub fn add_total(n: i32);
}
