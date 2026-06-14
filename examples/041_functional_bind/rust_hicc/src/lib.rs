//! 041_functional_bind: std::bind
//!
//! hicc 模式：与 040 同样使用 `hicc::Function<fn(i32) -> i32>`，但 C++ 侧函数
//! 是 `std::bind` 的结果（`std::function<int(int)>`）。从 FFI 角度看，这与 040 完全等价：
//! std::bind 出来的可调用对象在 C++ 侧赋值给 `std::function<int(int)>`，
//! hicc 直接以其类型签名暴露即可。
//!
//! 关键点：std::bind 不影响 FFI 形状 —— 只要 C++ 把 bind 结果作为
//! `std::function<R(Args...)>` 返回，Rust 侧就能用 `hicc::Function<fn(...) -> R>` 接收。

hicc::cpp! {
    #include "functional_bind.h"
}

hicc::import_class! {
    #[cpp(class = "functional_bind_ns::BoundAccumulator")]
    pub class BoundAccumulator {
        #[cpp(method = "int call_and_accumulate(int)")]
        pub fn call_and_accumulate(&mut self, x: i32) -> i32;

        #[cpp(method = "int base() const")]
        pub fn base(&self) -> i32;

        #[cpp(method = "void reset(int)")]
        pub fn reset(&mut self, v: i32);

        pub fn new(fn_: hicc::Function<fn(i32) -> i32>) -> Self { make_accumulator(fn_) }
    }
}

hicc::import_lib! {
    #![link_name = "functional_bind"]

    #[cpp(func = "std::function<int(int)> functional_bind_ns::make_adder(int)")]
    pub fn make_adder(n: i32) -> hicc::Function<fn(i32) -> i32>;

    #[cpp(func = "std::function<int(int)> functional_bind_ns::make_multiplier(int)")]
    pub fn make_multiplier(n: i32) -> hicc::Function<fn(i32) -> i32>;

    #[cpp(func = "std::function<int(int)> functional_bind_ns::make_subtractor(int)")]
    pub fn make_subtractor(n: i32) -> hicc::Function<fn(i32) -> i32>;

    #[cpp(func = "int functional_bind_ns::apply_bound(std::function<int(int)>, int)")]
    pub fn apply_bound(fn_: hicc::Function<fn(i32) -> i32>, x: i32) -> i32;

    #[cpp(func = "std::function<int(int)> functional_bind_ns::compose(std::function<int(int)>, std::function<int(int)>)")]
    pub fn compose(
        outer: hicc::Function<fn(i32) -> i32>,
        inner: hicc::Function<fn(i32) -> i32>,
    ) -> hicc::Function<fn(i32) -> i32>;

    #[cpp(func = "std::unique_ptr<functional_bind_ns::BoundAccumulator> functional_bind_ns::make_accumulator(std::function<int(int)>)")]
    pub fn make_accumulator(fn_: hicc::Function<fn(i32) -> i32>) -> BoundAccumulator;
}
