//! 033_raii_pattern: RAII / Drop
//!
//! hicc 模式：使用 `import_class!` + `make_unique` 工厂。
//! Rust 端 FileHandle 离开作用域时 Drop = C++ unique_ptr 的释放
//! （等价于触发 C++ FileHandle 的析构 = RAII 资源回收）。
//! 因此不需要绑 `consume_*` —— Rust Drop 自动表达 RAII。

hicc::cpp! {
    #include "raii_pattern.h"
    #include <hicc/std/string.hpp>
}

hicc::import_class! {
    class string = hicc_std::string;

    #[cpp(class = "raii_pattern_ns::FileHandle")]
    pub class FileHandle {
        #[cpp(method = "int fd() const")]
        pub fn fd(&self) -> i32;

        #[cpp(method = "const std::string& path() const")]
        pub fn path(&self) -> &string;

        #[cpp(method = "long write(const std::string&)")]
        pub fn write(&mut self, data: &string) -> i64;

        #[cpp(method = "long size() const")]
        pub fn size(&self) -> i64;

        pub fn open(fd: i32, path: &string) -> Self { open_file(fd, path) }
    }
}

hicc::import_lib! {
    #![link_name = "raii_pattern"]

    #[cpp(func = "std::unique_ptr<raii_pattern_ns::FileHandle> hicc::make_unique<raii_pattern_ns::FileHandle, int, const std::string&>(int&&, const std::string&)")]
    pub fn open_file(fd: i32, path: &hicc_std::string) -> FileHandle;

    #[cpp(func = "long raii_pattern_ns::read_file(raii_pattern_ns::FileHandle&)")]
    pub fn read_file(h: &mut FileHandle) -> i64;
}
