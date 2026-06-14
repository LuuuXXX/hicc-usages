//! 047_noexcept_basic: noexcept is transparent
//!
//! hicc 模式：`noexcept` 是 C++ 编译期/运行时合约，对 FFI ABI **完全透明**。
//! hicc-build 不感知 `noexcept` 关键字，按普通函数绑定即可。
//!
//! 关键点：C++ 标 `noexcept` 的函数不会抛异常（即使真的抛，也是 `std::terminate`），
//! 所以 Rust 端**不需要** `hicc::Exception<T>` 包装 —— 直接绑定即可。
//!
//! 对比：`may_throw(int)` 不标 `noexcept` 且真的抛异常 → 若想 Rust 端安全捕获，
//! 需用 `hicc::Exception<T>`。本例为简化起见把它也按普通函数绑，但避免在 Rust 端触发负值。

hicc::cpp! {
    #include "noexcept_basic.h"
    #include <hicc/std/string.hpp>
}

hicc::import_class! {
    #[cpp(class = "noexcept_basic_ns::SafeCounter")]
    pub class SafeCounter {
        #[cpp(method = "void increment(int)")]
        pub fn increment(&mut self, by: i32);

        #[cpp(method = "int get() const")]
        pub fn get(&self) -> i32;

        #[cpp(method = "void reset()")]
        pub fn reset(&mut self);

        #[cpp(method = "std::string describe() const")]
        pub fn describe(&self) -> hicc_std::string;

        pub fn new() -> Self { make_counter() }
    }
}

hicc::import_class! {
    #[cpp(class = "noexcept_basic_ns::Buffer")]
    pub class Buffer {
        #[cpp(method = "size_t size() const")]
        pub fn size(&self) -> usize;

        #[cpp(method = "int get(size_t) const")]
        pub fn get(&self, idx: usize) -> i32;

        #[cpp(method = "void set(size_t, int)")]
        pub fn set(&mut self, idx: usize, value: i32);

        pub fn new(n: usize) -> Self { make_buffer(n) }
    }
}

hicc::import_lib! {
    #![link_name = "noexcept_basic"]

    class string = hicc_std::string;

    #[cpp(func = "int noexcept_basic_ns::add_noexcept(int, int)")]
    pub fn add_noexcept(a: i32, b: i32) -> i32;

    #[cpp(func = "int noexcept_basic_ns::square_noexcept(int)")]
    pub fn square_noexcept(x: i32) -> i32;

    #[cpp(func = "double noexcept_basic_ns::safe_reciprocal_noexcept(double)")]
    pub fn safe_reciprocal_noexcept(x: f64) -> f64;

    #[cpp(func = "int noexcept_basic_ns::compute_constant()")]
    pub fn compute_constant() -> i32;

    #[cpp(func = "int noexcept_basic_ns::may_throw(int)")]
    pub fn may_throw(x: i32) -> hicc::Exception<i32>;

    #[cpp(func = "std::unique_ptr<noexcept_basic_ns::SafeCounter> noexcept_basic_ns::make_counter()")]
    pub fn make_counter() -> SafeCounter;

    #[cpp(func = "std::unique_ptr<noexcept_basic_ns::Buffer> noexcept_basic_ns::make_buffer(size_t)")]
    pub fn make_buffer(n: usize) -> Buffer;
}
