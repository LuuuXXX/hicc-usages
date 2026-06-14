// hicc routes C++ throw/catch through Exception<T>: success value or
// ExceptionInfo (what() string). Rust caller calls .ok() → Result<T, _>.

hicc::cpp! {
    #include "exception_basic.h"
}

hicc::import_lib! {
    #![link_name = "exception_basic_hicc"]

    #[cpp(func = "int safe_divide(int, int)")]
    pub fn safe_divide(a: i32, b: i32) -> i32;

    #[cpp(func = "int throwing_divide(int, int)")]
    pub fn throwing_divide(a: i32, b: i32) -> hicc::Exception<i32>;

    #[cpp(func = "int checked_index(const int*, int, int)")]
    pub fn checked_index(arr: *const i32, n: i32, i: i32) -> hicc::Exception<i32>;
}
