//! 034_vector_basic: std::vector
//!
//! hicc 模式：`hicc_std::vector<T>` 别名 + `make_unique` 工厂。
//! 在 cpp! 块 typedef 容器类型；在 import_lib! 给定别名（RustVec）和工厂。
//! Rust 端通过 hicc_std::vector 的内置方法（push_back / size / as_slice 等）使用。

hicc::cpp! {
    #include "vector_basic.h"
    #include <hicc/std/vector.hpp>
    // 实例化容器类型（C++ 侧需要展开 hicc/std 头文件中的模板）
    typedef std::vector<int> CppVec;
}

hicc::import_lib! {
    #![link_name = "vector_basic"]

    // Rust 端别名：T 为 Pod<i32>（POD 必须包一层）
    class RustVec = hicc_std::vector<hicc::Pod<i32>>;

    // 工厂：通过 make_unique 创建空容器
    #[cpp(func = "std::unique_ptr<CppVec> hicc::make_unique<CppVec>()")]
    pub fn vec_new() -> RustVec;

    // 业务函数：sum
    #[cpp(func = "long vector_basic_ns::vector_sum(const std::vector<int>&)")]
    pub fn vector_sum(v: &RustVec) -> i64;

    // 业务函数：avg
    #[cpp(func = "double vector_basic_ns::vector_avg(const std::vector<int>&)")]
    pub fn vector_avg(v: &RustVec) -> f64;
}
