//! 001_hello_world: 通过 hicc 调用最简单的 C++ 全局函数。
//!
//! hicc 模式：`import_lib!` + `#[cpp(func = ...)]`

hicc::cpp! {
    #include "hello_world.h"
}

hicc::import_lib! {
    #![link_name = "hello_world"]

    #[cpp(func = "void hello_world_ns::hello_world()")]
    pub fn hello_world();

    #[cpp(func = "int hello_world_ns::answer()")]
    pub fn answer() -> i32;
}
