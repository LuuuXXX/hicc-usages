//! 019_operator_overload: C++ operator 重载
//!
//! hicc 模式：hicc **不直接支持 operator+/-/* 等重载**（method 名带特殊符号）。
//! 用 cpp! 块写命名空间级 inline 包装函数（如 `vec_add`），Rust 端通过包装调用。
//! 普通 accessor (x/y) 走 import_class! 的 method。

hicc::cpp! {
    #include "operator_overload.h"
    #include <hicc/std/string.hpp>

    inline operator_overload_ns::Vec2 vec_add(const operator_overload_ns::Vec2& a, const operator_overload_ns::Vec2& b) { return a + b; }
    inline operator_overload_ns::Vec2 vec_sub(const operator_overload_ns::Vec2& a, const operator_overload_ns::Vec2& b) { return a - b; }
    inline operator_overload_ns::Vec2 vec_scale(const operator_overload_ns::Vec2& a, float s) { return a * s; }
    inline operator_overload_ns::Vec2 vec_neg(const operator_overload_ns::Vec2& a) { return -a; }
    inline bool vec_eq(const operator_overload_ns::Vec2& a, const operator_overload_ns::Vec2& b) { return a == b; }
    inline float vec_at(const operator_overload_ns::Vec2& a, int i) { return a[i]; }
    inline void vec_iadd(operator_overload_ns::Vec2& a, const operator_overload_ns::Vec2& b) { a += b; }
}

hicc::import_class! {
    #[cpp(class = "operator_overload_ns::Vec2")]
    pub class Vec2 {
        #[cpp(method = "float x() const")]
        pub fn x(&self) -> f32;

        #[cpp(method = "float y() const")]
        pub fn y(&self) -> f32;

        pub fn new(x: f32, y: f32) -> Self { vec2_new(x, y) }
    }
}

hicc::import_lib! {
    #![link_name = "operator_overload"]

    #[cpp(func = "std::unique_ptr<operator_overload_ns::Vec2> hicc::make_unique<operator_overload_ns::Vec2, float, float>(float&&, float&&)")]
    pub fn vec2_new(x: f32, y: f32) -> Vec2;

    #[cpp(func = "operator_overload_ns::Vec2 vec_add(const operator_overload_ns::Vec2&, const operator_overload_ns::Vec2&)")]
    pub fn vec_add(a: &Vec2, b: &Vec2) -> Vec2;

    #[cpp(func = "operator_overload_ns::Vec2 vec_sub(const operator_overload_ns::Vec2&, const operator_overload_ns::Vec2&)")]
    pub fn vec_sub(a: &Vec2, b: &Vec2) -> Vec2;

    #[cpp(func = "operator_overload_ns::Vec2 vec_scale(const operator_overload_ns::Vec2&, float)")]
    pub fn vec_scale(a: &Vec2, s: f32) -> Vec2;

    #[cpp(func = "operator_overload_ns::Vec2 vec_neg(const operator_overload_ns::Vec2&)")]
    pub fn vec_neg(a: &Vec2) -> Vec2;

    #[cpp(func = "bool vec_eq(const operator_overload_ns::Vec2&, const operator_overload_ns::Vec2&)")]
    pub fn vec_eq(a: &Vec2, b: &Vec2) -> bool;

    #[cpp(func = "float vec_at(const operator_overload_ns::Vec2&, int)")]
    pub fn vec_at(a: &Vec2, i: i32) -> f32;

    #[cpp(func = "void vec_iadd(operator_overload_ns::Vec2&, const operator_overload_ns::Vec2&)")]
    pub fn vec_iadd(a: &mut Vec2, b: &Vec2);
}
