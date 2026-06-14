//! 005_variadic_functions: C 风格 `...` 与 `va_list`。
//!
//! hicc 模式：
//! - C 风格 `...` → Rust `unsafe fn f(arg0: T0, ...)`，调用方式 `f(t0)(t1, t2, ...)`
//! - `va_list` 作为最后一个参数 → Rust `unsafe fn f(..., ...)`，同样模式

hicc::cpp! {
    #include "variadic_functions.h"
}

hicc::import_lib! {
    #![link_name = "variadic_functions"]

    class variadic_ns;

    // C 风格变长参数：count 后跟 ...
    #[cpp(func = "int variadic_ns::sum_ints(int, ...)")]
    pub unsafe fn sum_ints(count: i32, ...) -> i32;

    // printf 风格
    #[cpp(func = "void variadic_ns::log_line(const char*, ...)")]
    pub unsafe fn log_line(fmt: *const i8, ...);
}
