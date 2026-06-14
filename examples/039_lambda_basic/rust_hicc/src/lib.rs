// Lambda → named free function on the C++ side. Rust binds the named wrapper.

hicc::cpp! {
    #include "lambda_basic.h"
}

hicc::import_lib! {
    #![link_name = "lambda_basic_hicc"]

    #[cpp(func = "int double_it(int)")]
    pub fn double_it(x: i32) -> i32;

    #[cpp(func = "int add_then_double(int, int)")]
    pub fn add_then_double(a: i32, b: i32) -> i32;

    #[cpp(func = "int sum_with_offset(int*, int, int)")]
    pub fn sum_with_offset(arr: *mut i32, n: i32, offset: i32) -> i32;
}
