//! 037_array_basic: std::array
//!
//! hicc 模式：`hicc_std::array<Pod<T>>` 别名 + `make_unique<std::array<T, N>>` 工厂。
//! 注意：`std::array<T, N>` 的 N 在模板参数里。hicc-std 的 `array` 是模板类，N 由具体
//! 工厂的 `make_unique<std::array<int, 5>>` 实例化决定。
//!
//! 业务函数参数 / 返回值为 `std::array<int, 5>&` 时 Rust 端用别名 `&RustArr5` 引用。

hicc::cpp! {
    #include "array_basic.h"
    #include <hicc/std/array.hpp>
    typedef std::array<int, 5> CppArr5;
}

hicc::import_lib! {
    #![link_name = "array_basic"]

    // Rust 端别名 —— T 为 Pod<i32>，N 由工厂决定
    pub class RustArr5 = hicc_std::array<hicc::Pod<i32>>;

    // 工厂：明确实例化 std::array<int, 5>
    #[cpp(func = "std::unique_ptr<CppArr5> hicc::make_unique<CppArr5>()")]
    pub fn array5_new() -> RustArr5;

    // 业务函数
    #[cpp(func = "long array_basic_ns::array_sum(const std::array<int, 5>&)")]
    pub fn array_sum(a: &RustArr5) -> i64;

    #[cpp(func = "int array_basic_ns::array_max(const std::array<int, 5>&)")]
    pub fn array_max(a: &RustArr5) -> i32;

    #[cpp(func = "double array_basic_ns::array_avg(const std::array<int, 5>&)")]
    pub fn array_avg(a: &RustArr5) -> f64;

    #[cpp(func = "void array_basic_ns::fill_array(std::array<int, 5>&, int)")]
    pub fn fill_array(a: &mut RustArr5, start: i32);
}
