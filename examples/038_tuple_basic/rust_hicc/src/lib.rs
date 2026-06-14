//! 038_tuple_basic: std::tuple
//!
//! hicc 模式：⚠️ std::tuple 不能直接 FFI（无 ABI 稳定布局，无内置模板特化）。
//! 解决方案：
//! 1. C++ 端工厂返回 `std::unique_ptr<Triple>` —— hicc 把它当作 Triple 不透明对象
//!    （默认 deleter → 直接绑类对象）。
//! 2. 每个字段访问 / 修改通过命名空间级自由函数（triple_id / triple_name / triple_score /
//!    set_id / set_score），这些函数内部用 `std::get<I>(t)`。
//! 3. Rust 端 import_class! 声明 Triple 为不透明类（无方法），import_lib! 绑工厂 + 访问器。
//!
//! 等价于手工写 `std::get<I>(t)` 的 Rust 绑定 —— 自动化时按字段索引生成访问器。

hicc::cpp! {
    #include "tuple_basic.h"
    #include <hicc/std/string.hpp>
}

hicc::import_class! {
    class string = hicc_std::string;

    // Triple 是不透明类型 —— 无 Rust 端方法；所有访问都通过自由函数
    #[cpp(class = "tuple_basic_ns::Triple")]
    pub class Triple {
    }
}

hicc::import_lib! {
    #![link_name = "tuple_basic"]

    #[cpp(func = "std::unique_ptr<tuple_basic_ns::Triple> tuple_basic_ns::make_triple(int, const std::string&, double)")]
    pub fn make_triple(id: i32, name: &hicc_std::string, score: f64) -> Triple;

    #[cpp(func = "int tuple_basic_ns::triple_id(const tuple_basic_ns::Triple&)")]
    pub fn triple_id(t: &Triple) -> i32;

    #[cpp(func = "std::string tuple_basic_ns::triple_name(const tuple_basic_ns::Triple&)")]
    pub fn triple_name(t: &Triple) -> hicc_std::string;

    #[cpp(func = "double tuple_basic_ns::triple_score(const tuple_basic_ns::Triple&)")]
    pub fn triple_score(t: &Triple) -> f64;

    #[cpp(func = "void tuple_basic_ns::set_id(tuple_basic_ns::Triple&, int)")]
    pub fn set_id(t: &mut Triple, id: i32);

    #[cpp(func = "void tuple_basic_ns::set_score(tuple_basic_ns::Triple&, double)")]
    pub fn set_score(t: &mut Triple, score: f64);
}
