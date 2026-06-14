//! 018_virtual_diamond: 菱形虚继承（virtual base）
//!
//! hicc 模式：菱形虚继承导致 `&IOCombo::method` 的指针到成员转换报
//! "pointer to member conversion via virtual base"。**所有方法**都通过 cpp! 块的
//! inline 自由函数包装，import_lib! 直接绑自由函数；import_class! 仅承载类型 + 工厂。

hicc::cpp! {
    #include "virtual_diamond.h"
    #include <hicc/std/string.hpp>

    inline std::string diamond_id(const virtual_diamond_ns::IOCombo& c) { return c.id(); }
    inline std::string diamond_category(const virtual_diamond_ns::IOCombo& c) { return c.category(); }
    inline int diamond_read(virtual_diamond_ns::IOCombo& c) { return c.read(); }
    inline void diamond_write(virtual_diamond_ns::IOCombo& c, int v) { c.write(v); }
    inline int diamond_last_input(const virtual_diamond_ns::IOCombo& c) { return c.last_input(); }
    inline int diamond_last_output(const virtual_diamond_ns::IOCombo& c) { return c.last_output(); }
}

hicc::import_class! {
    class string = hicc_std::string;

    #[cpp(class = "virtual_diamond_ns::IOCombo")]
    pub class IOCombo {
        pub fn new(id: &string) -> Self { iocombo_new(id) }

        pub fn id(&self) -> string { diamond_id(self) }
        pub fn category(&self) -> string { diamond_category(self) }
        pub fn read(&mut self) -> i32 { diamond_read(self) }
        pub fn write(&mut self, v: i32) { diamond_write(self, v) }
        pub fn last_input(&self) -> i32 { diamond_last_input(self) }
        pub fn last_output(&self) -> i32 { diamond_last_output(self) }
    }
}

hicc::import_lib! {
    #![link_name = "virtual_diamond"]

    #[cpp(func = "std::unique_ptr<virtual_diamond_ns::IOCombo> hicc::make_unique<virtual_diamond_ns::IOCombo, const std::string&>(const std::string&)")]
    pub fn iocombo_new(id: &hicc_std::string) -> IOCombo;

    #[cpp(func = "std::string diamond_id(const virtual_diamond_ns::IOCombo&)")]
    pub fn diamond_id(c: &IOCombo) -> string;

    #[cpp(func = "std::string diamond_category(const virtual_diamond_ns::IOCombo&)")]
    pub fn diamond_category(c: &IOCombo) -> string;

    #[cpp(func = "int diamond_read(virtual_diamond_ns::IOCombo&)")]
    pub fn diamond_read(c: &mut IOCombo) -> i32;

    #[cpp(func = "void diamond_write(virtual_diamond_ns::IOCombo&, int)")]
    pub fn diamond_write(c: &mut IOCombo, v: i32);

    #[cpp(func = "int diamond_last_input(const virtual_diamond_ns::IOCombo&)")]
    pub fn diamond_last_input(c: &IOCombo) -> i32;

    #[cpp(func = "int diamond_last_output(const virtual_diamond_ns::IOCombo&)")]
    pub fn diamond_last_output(c: &IOCombo) -> i32;
}
