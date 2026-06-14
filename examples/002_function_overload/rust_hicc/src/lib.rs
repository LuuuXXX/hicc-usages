// Bind the overloaded C++ add(int, int) and add(double, double).
// Rust has no overloading, so rename to add_i32 / add_f64. The hicc #[cpp(func=...)]
// attribute carries the original C++ signature for type-matching.

hicc::cpp! {
    #include "function_overload.h"
}

hicc::import_lib! {
    #![link_name = "function_overload_hicc"]

    #[cpp(func = "int add(int, int)")]
    pub fn add_i32(a: i32, b: i32) -> i32;

    #[cpp(func = "double add(double, double)")]
    pub fn add_f64(a: f64, b: f64) -> f64;
}
