// Operator overloading: named wrappers (vec2_add / vec2_sub / vec2_eq) are
// bound as ordinary free functions. The C++ operators themselves stay in C++.

// Comment-style injection suggestion:
// rust_gen could detect `operator+` and auto-generate `vec2_add` wrapper
// + the corresponding `#[cpp(func = "...")]` binding.

hicc::cpp! {
    #include "operator_overload.h"
}

hicc::import_class! {
    #[cpp(class = "Vec2", destroy = "vec2_free")]
    pub class Vec2 {
        #[cpp(method = "int x() const")]
        pub fn x(&self) -> i32;
        #[cpp(method = "int y() const")]
        pub fn y(&self) -> i32;
    }
}

hicc::import_lib! {
    #![link_name = "operator_overload_hicc"]

    #[cpp(func = "Vec2* vec2_new(int, int)")]
    pub fn vec2_new(x: i32, y: i32) -> Vec2;

    // Wrapper for operator+ (returns by value → Rust receives an owned Vec2)
    #[cpp(func = "Vec2 vec2_add(const Vec2&, const Vec2&)")]
    pub fn vec2_add(a: &Vec2, b: &Vec2) -> Vec2;

    #[cpp(func = "Vec2 vec2_sub(const Vec2&, const Vec2&)")]
    pub fn vec2_sub(a: &Vec2, b: &Vec2) -> Vec2;

    #[cpp(func = "bool vec2_eq(const Vec2&, const Vec2&)")]
    pub fn vec2_eq(a: &Vec2, b: &Vec2) -> bool;
}
