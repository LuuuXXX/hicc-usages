//! 自动生成：hicc_usage_inheritance_multiple
//!
//! 由 tools/rust-gen/rust_gen.py 从 ../ast/symbols.json 生成。
//! 请勿手动编辑；如需调整请修改 tools/rust-gen/special.yaml 后重新生成。

hicc::cpp! {
    #include "hicc_usages/inheritance_multiple.h"
}
hicc::import_class! {
    #[cpp(class = "hicc_usages::inheritance_multiple::Drawable")]
    pub class Drawable {
        #[cpp(method = "const char * shape_name() const")]
        pub fn shape_name(&self) -> *const i8;
    }
}
hicc::import_class! {
    #[cpp(class = "hicc_usages::inheritance_multiple::Printable")]
    pub class Printable {
        #[cpp(method = "const char * printable_text() const")]
        pub fn printable_text(&self) -> *const i8;
    }
}
hicc::import_class! {
    #[cpp(class = "hicc_usages::inheritance_multiple::Shape", destroy = "hicc_usages::inheritance_multiple::Shape::free")]
    pub class Shape {
        #[cpp(method = "const char * shape_name() const")]
        pub fn shape_name(&self) -> *const i8;
        #[cpp(method = "const char * printable_text() const")]
        pub fn printable_text(&self) -> *const i8;
    }
}
hicc::import_lib! {
    #![link_name = "hicc_usage_inheritance_multiple_adapter"]
    pub class Drawable;
    pub class Printable;
    pub class Shape;
    #[cpp(func = "int hicc_usages::inheritance_multiple::Drawable::draw_calls()")]
    pub fn drawable_draw_calls() -> i32;
    #[cpp(func = "int hicc_usages::inheritance_multiple::Printable::print_calls()")]
    pub fn printable_print_calls() -> i32;
    #[cpp(func = "hicc_usages::inheritance_multiple::Shape * hicc_usages::inheritance_multiple::Shape::create()")]
    pub fn shape_new() -> Shape;
}
