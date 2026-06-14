// C-style variadic int sum(int n, ...) cannot cross FFI directly. We bind the
// fixed-arity wrappers (sum2 / sum3) instead — the C++ side keeps the variadic
// version for C++ callers.

hicc::cpp! {
    #include "variadic_functions.h"
}

hicc::import_lib! {
    #![link_name = "variadic_functions_hicc"]

    #[cpp(func = "int sum2(int, int)")]
    pub fn sum2(a: i32, b: i32) -> i32;

    #[cpp(func = "int sum3(int, int, int)")]
    pub fn sum3(a: i32, b: i32, c: i32) -> i32;
}
