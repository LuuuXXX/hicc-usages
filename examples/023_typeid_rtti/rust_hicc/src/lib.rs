// RTTI: typeid().name() is not FFI-safe. We expose named accessors
// (`type_name_of`, `static_type_name_*`) that return `const char*`.

hicc::cpp! {
    #include "typeid_rtti.h"
}

hicc::import_class! {
    #[cpp(class = "Circle", destroy = "shape_free_circle")]
    pub class Circle {
        #[cpp(method = "int area() const")]
        pub fn area(&self) -> i32;
    }
}

hicc::import_class! {
    #[cpp(class = "Triangle", destroy = "shape_free_triangle")]
    pub class Triangle {
        #[cpp(method = "int area() const")]
        pub fn area(&self) -> i32;
    }
}

hicc::import_lib! {
    #![link_name = "typeid_rtti_hicc"]

    #[cpp(func = "Circle* circle_new(int)")]
    pub fn circle_new(r: i32) -> Circle;

    #[cpp(func = "Triangle* triangle_new(int, int)")]
    pub fn triangle_new(b: i32, h: i32) -> Triangle;

    #[cpp(func = "const char* type_name_of(const Shape*)")]
    pub fn type_name_of_circle(c: &Circle) -> *const i8;

    #[cpp(func = "const char* type_name_of(const Shape*)")]
    pub fn type_name_of_triangle(t: &Triangle) -> *const i8;

    #[cpp(func = "const char* static_type_name_circle()")]
    pub fn static_type_name_circle() -> *const i8;

    #[cpp(func = "const char* static_type_name_triangle()")]
    pub fn static_type_name_triangle() -> *const i8;
}
