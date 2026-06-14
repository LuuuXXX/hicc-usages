// std::bind return type is not nameable across FFI — bind named wrappers.

hicc::cpp! {
    #include "functional_bind.h"
}

hicc::import_class! {
    #[cpp(class = "BindPoint", destroy = "bind_point_free")]
    pub class BindPoint {
    }
}

hicc::import_lib! {
    #![link_name = "functional_bind_hicc"]

    #[cpp(func = "BindPoint* bind_point_new(int, int)")]
    pub fn bind_point_new(x: i32, y: i32) -> BindPoint;

    #[cpp(func = "int add_bound_10(int)")]
    pub fn add_bound_10(x: i32) -> i32;

    #[cpp(func = "int mul_bound_3(int)")]
    pub fn mul_bound_3(x: i32) -> i32;

    #[cpp(func = "int sub_bind_first(int, int)")]
    pub fn sub_bind_first(a: i32, b: i32) -> i32;

    #[cpp(func = "int point_x_plus_offset(const BindPoint*, int)")]
    pub fn point_x_plus_offset(p: *const BindPoint, offset: i32) -> i32;
}
