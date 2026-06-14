//! 025_template_class: 模板类
//!
//! hicc 模式：模板类不能直接绑。cpp! 块为每个具现化类型 `using` 一个别名，
//! import_class! 用别名导入。`push(const T&)` 对原始类型 int 需 cpp! 包装。

hicc::cpp! {
    #include "template_class.h"
    #include <hicc/std/string.hpp>

    using StackInt = template_class_ns::Stack<int>;
    using StackString = template_class_ns::Stack<std::string>;

    inline void stack_int_push_wrap(StackInt& s, int v) { s.push(v); }
    inline void stack_string_push_wrap(StackString& s, const std::string& v) { s.push(v); }
}

hicc::import_class! {
    class string = hicc_std::string;

    #[cpp(class = "StackInt")]
    pub class StackInt {
        #[cpp(method = "void pop()")]
        pub fn pop(&mut self);

        #[cpp(method = "int top() const")]
        pub fn top(&self) -> i32;

        #[cpp(method = "size_t size() const")]
        pub fn size(&self) -> usize;

        #[cpp(method = "bool empty() const")]
        pub fn empty(&self) -> bool;

        pub fn new() -> Self { stack_int_new() }
        pub fn push(&mut self, v: i32) { stack_int_push(self, v) }
    }

    #[cpp(class = "StackString")]
    pub class StackString {
        #[cpp(method = "void pop()")]
        pub fn pop(&mut self);

        #[cpp(method = "std::string top() const")]
        pub fn top(&self) -> string;

        #[cpp(method = "size_t size() const")]
        pub fn size(&self) -> usize;

        pub fn new() -> Self { stack_string_new() }
        pub fn push(&mut self, v: &string) { stack_string_push(self, v) }
    }
}

hicc::import_lib! {
    #![link_name = "template_class"]

    #[cpp(func = "std::unique_ptr<StackInt> hicc::make_unique<StackInt>()")]
    pub fn stack_int_new() -> StackInt;

    #[cpp(func = "std::unique_ptr<StackString> hicc::make_unique<StackString>()")]
    pub fn stack_string_new() -> StackString;

    #[cpp(func = "void stack_int_push_wrap(StackInt&, int)")]
    pub fn stack_int_push(s: &mut StackInt, v: i32);

    #[cpp(func = "void stack_string_push_wrap(StackString&, const std::string&)")]
    pub fn stack_string_push(s: &mut StackString, v: &hicc_std::string);
}
