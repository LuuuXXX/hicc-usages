//! 045_union_basic: union via wrapper accessors
//!
//! hicc 模式：C++ 的 `union Value { int i; float f; long l; }` 是 POD（trivially copyable），
//! 但 hicc 不能直接表达 union 字段。两种应对方式：
//!
//! 1. **顶层自由函数（POD union 按值传）**：把 `Value` 当成 opaque 字节块，
//!    Rust 端用一个 `#[repr(C)] struct Value([u8; 8])` 镜像（long 是 8 字节，覆盖整个 union），
//!    跨 FFI 时按值传 8 字节。
//! 2. **类内成员**：`Box` 内部存 `Value`，但所有方法（`as_int`/`as_float`/`as_long`/`set_*`）
//!    都返回/接收具体类型（int/float/long），不暴露 `Value` 本身。
//!
//! 这样 Rust 端永远见不到 union 字段，只看到具体类型，符合 hicc FFI 的"具体类型"约束。

hicc::cpp! {
    #include "union_basic.h"
    #include <hicc/std/string.hpp>
}

/// Mirror of C++ `union Value { int i; float f; long l; }` as opaque bytes.
/// POD and trivially copyable — pass by value through FFI (8 bytes).
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Value(pub [u8; 8]);

hicc::import_class! {
    #[cpp(class = "union_basic_ns::Box")]
    pub class Box {
        // Tag returned as int (mirror of enum class Tag)
        #[cpp(method = "union_basic_ns::Tag tag() const")]
        pub fn tag_raw(&self) -> i32;

        #[cpp(method = "int as_int() const")]
        pub fn as_int(&self) -> i32;

        #[cpp(method = "float as_float() const")]
        pub fn as_float(&self) -> f32;

        #[cpp(method = "long as_long() const")]
        pub fn as_long(&self) -> i64;

        #[cpp(method = "void set_int(int)")]
        pub fn set_int(&mut self, x: i32);

        #[cpp(method = "void set_float(float)")]
        pub fn set_float(&mut self, x: f32);

        #[cpp(method = "void set_long(long)")]
        pub fn set_long(&mut self, x: i64);

        #[cpp(method = "std::string describe() const")]
        pub fn describe(&self) -> hicc_std::string;

        pub fn new_int(x: i32) -> Self { make_box_int(x) }
        pub fn new_float(x: f32) -> Self { make_box_float(x) }
        pub fn new_long(x: i64) -> Self { make_box_long(x) }
    }
}

hicc::import_lib! {
    #![link_name = "union_basic"]

    class string = hicc_std::string;

    // POD union passed by value (8 bytes)
    #[cpp(func = "int union_basic_ns::value_as_int(union_basic_ns::Value)")]
    pub fn value_as_int(v: Value) -> i32;

    #[cpp(func = "float union_basic_ns::value_as_float(union_basic_ns::Value)")]
    pub fn value_as_float(v: Value) -> f32;

    #[cpp(func = "long union_basic_ns::value_as_long(union_basic_ns::Value)")]
    pub fn value_as_long(v: Value) -> i64;

    #[cpp(func = "union_basic_ns::Value union_basic_ns::make_value_int(int)")]
    pub fn make_value_int(x: i32) -> Value;

    #[cpp(func = "union_basic_ns::Value union_basic_ns::make_value_float(float)")]
    pub fn make_value_float(x: f32) -> Value;

    #[cpp(func = "union_basic_ns::Value union_basic_ns::make_value_long(long)")]
    pub fn make_value_long(x: i64) -> Value;

    #[cpp(func = "union_basic_ns::Box union_basic_ns::make_box_int(int)")]
    pub fn make_box_int(x: i32) -> Box;

    #[cpp(func = "union_basic_ns::Box union_basic_ns::make_box_float(float)")]
    pub fn make_box_float(x: f32) -> Box;

    #[cpp(func = "union_basic_ns::Box union_basic_ns::make_box_long(long)")]
    pub fn make_box_long(x: i64) -> Box;
}

// ---- Tag enum mirror (same pattern as 044) ----

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tag { Int, Float, Long }

impl Box {
    pub fn tag(&self) -> Tag {
        match self.tag_raw() {
            0 => Tag::Int,
            1 => Tag::Float,
            _ => Tag::Long,
        }
    }
}
