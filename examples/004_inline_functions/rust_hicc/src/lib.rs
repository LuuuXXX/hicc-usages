//! 004_inline_functions: inline/constexpr 函数对 FFI 透明，与普通函数同模式。

hicc::cpp! {
    #include "inline_functions.h"
}

hicc::import_lib! {
    #![link_name = "inline_functions"]

    class inline_ns;

    #[cpp(func = "int inline_ns::square(int)")]
    pub fn square(x: i32) -> i32;

    #[cpp(func = "int inline_ns::cube(int)")]
    pub fn cube(x: i32) -> i32;

    #[cpp(func = "int inline_ns::factorial(int)")]
    pub fn factorial(n: i32) -> i32;
}
