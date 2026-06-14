// std::function is not nameable across FFI — bind named C++ wrappers.

hicc::cpp! {
    #include "std_function.h"
}

hicc::import_lib! {
    #![link_name = "std_function_hicc"]

    #[cpp(func = "int add_op(int, int)")]
    pub fn add_op(a: i32, b: i32) -> i32;

    #[cpp(func = "int mul_op(int, int)")]
    pub fn mul_op(a: i32, b: i32) -> i32;

    #[cpp(func = "int run_binary_op(int, int, int)")]
    pub fn run_binary_op(a: i32, b: i32, op_kind: i32) -> i32;

    #[cpp(func = "int compose_then_add_then_mul(int, int, int)")]
    pub fn compose_then_add_then_mul(x: i32, add_n: i32, mul_n: i32) -> i32;
}
