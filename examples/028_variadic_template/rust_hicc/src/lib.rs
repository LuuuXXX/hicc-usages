// Variadic template: Rust binds the fixed-arity wrappers (sum2/sum3/sum4).
// The original `sum_all<Ts...>` is unreachable from FFI.

hicc::cpp! {
    #include "variadic_template.h"
}

hicc::import_lib! {
    #![link_name = "variadic_template_hicc"]

    #[cpp(func = "int sum2(int, int)")]
    pub fn sum2(a: i32, b: i32) -> i32;

    #[cpp(func = "int sum3(int, int, int)")]
    pub fn sum3(a: i32, b: i32, c: i32) -> i32;

    #[cpp(func = "int sum4(int, int, int, int)")]
    pub fn sum4(a: i32, b: i32, c: i32, d: i32) -> i32;
}
