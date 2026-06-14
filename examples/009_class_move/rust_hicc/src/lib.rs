//! 009_class_move: 显式移动语义 + operator+=（用 cpp! 包装）
//!
//! hicc 模式：与 008 相同（copy/move 工厂），新增 operator+= 包装

hicc::cpp! {
    #include "class_move.h"
    #include <hicc/std/string.hpp>

    // hicc 不直接支持 operator+=，用包装函数
    inline void holder_add_to(class_move_ns::Holder& h, int delta) {
        h += delta;
    }
}

hicc::import_class! {
    class string = hicc_std::string;

    #[cpp(class = "class_move_ns::Holder")]
    pub class Holder {
        #[cpp(method = "int size() const")]
        pub fn size(&self) -> i32;

        #[cpp(method = "int first() const")]
        pub fn first(&self) -> i32;

        #[cpp(method = "const std::string& tag() const")]
        pub fn tag(&self) -> &string;

        pub fn new(sz: i32, tag: &string) -> Self { holder_new(sz, tag) }
        pub fn default_() -> Self { holder_default() }
        pub fn clone(&self) -> Self { holder_clone(self) }
        pub fn move_from(other: Self) -> Self { holder_move(other) }
        pub fn add_to(&mut self, delta: i32) { holder_add_to(self, delta) }
    }
}

hicc::import_lib! {
    #![link_name = "class_move"]

    #[cpp(func = "std::unique_ptr<class_move_ns::Holder> hicc::make_unique<class_move_ns::Holder, int, const std::string&>(int&&, const std::string&)")]
    pub fn holder_new(sz: i32, tag: &hicc_std::string) -> Holder;

    #[cpp(func = "std::unique_ptr<class_move_ns::Holder> hicc::make_unique<class_move_ns::Holder>()")]
    pub fn holder_default() -> Holder;

    #[cpp(func = "std::unique_ptr<class_move_ns::Holder> hicc::make_unique<class_move_ns::Holder, const class_move_ns::Holder&>(const class_move_ns::Holder&)")]
    pub fn holder_clone(other: &Holder) -> Holder;

    #[cpp(func = "std::unique_ptr<class_move_ns::Holder> hicc::make_unique<class_move_ns::Holder, class_move_ns::Holder&&>(class_move_ns::Holder&&)")]
    pub fn holder_move(other: Holder) -> Holder;

    #[cpp(func = "void holder_add_to(class_move_ns::Holder&, int)")]
    pub fn holder_add_to(h: &mut Holder, delta: i32);
}
