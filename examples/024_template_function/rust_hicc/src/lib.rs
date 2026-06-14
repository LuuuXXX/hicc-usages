// Function template: bind each explicit instantiation with its concrete
// type parameter in `#[cpp(func = "...")]`.

hicc::cpp! {
    #include "template_function.h"
}

hicc::import_lib! {
    #![link_name = "template_function_hicc"]

    #[cpp(func = "int identity<int>(int)")]
    pub fn identity_i32(x: i32) -> i32;

    #[cpp(func = "double identity<double>(double)")]
    pub fn identity_f64(x: f64) -> f64;

    #[cpp(func = "int add_tmpl<int>(int, int)")]
    pub fn add_tmpl_i32(a: i32, b: i32) -> i32;

    #[cpp(func = "double add_tmpl<double>(double, double)")]
    pub fn add_tmpl_f64(a: f64, b: f64) -> f64;
}
