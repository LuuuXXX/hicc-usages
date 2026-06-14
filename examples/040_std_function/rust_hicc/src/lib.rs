//! 040_std_function: std::function
//!
//! hicc 模式：与 039 同样使用 `hicc::Function<fn(...) -> R>`。
//! 重点演示：把 std::function 作为**类成员**（Callback 内）持久化保存，并通过方法调用。
//!
//! 关键点：Callback 的 `replace(std::function<int(int)>)` 方法接受新的 lambda ——
//! hicc 端把方法绑成接收 `hicc::Function<fn(i32) -> i32>` 参数。

hicc::cpp! {
    #include "std_function.h"
}

hicc::import_class! {
    #[cpp(class = "std_function_ns::Callback")]
    pub class Callback {
        #[cpp(method = "int invoke(int) const")]
        pub fn invoke(&self, x: i32) -> i32;

        #[cpp(method = "void replace(std::function<int(int)>)")]
        pub fn replace(&mut self, fn_: hicc::Function<fn(i32) -> i32>);

        #[cpp(method = "long call_n_times(int, int) const")]
        pub fn call_n_times(&self, x: i32, n: i32) -> i64;

        pub fn new(fn_: hicc::Function<fn(i32) -> i32>) -> Self { make_callback(fn_) }
    }
}

hicc::import_lib! {
    #![link_name = "std_function"]

    #[cpp(func = "int std_function_ns::apply_dbl(std::function<int(int)>, int)")]
    pub fn apply_dbl(fn_: hicc::Function<fn(i32) -> i32>, x: i32) -> i32;

    #[cpp(func = "std::function<int(int)> std_function_ns::make_doubler()")]
    pub fn make_doubler() -> hicc::Function<fn(i32) -> i32>;

    #[cpp(func = "int std_function_ns::chain(std::function<int(int)>, std::function<int(int)>, int)")]
    pub fn chain(
        f: hicc::Function<fn(i32) -> i32>,
        g: hicc::Function<fn(i32) -> i32>,
        x: i32,
    ) -> i32;

    #[cpp(func = "std::unique_ptr<std_function_ns::Callback> std_function_ns::make_callback(std::function<int(int)>)")]
    pub fn make_callback(fn_: hicc::Function<fn(i32) -> i32>) -> Callback;
}
