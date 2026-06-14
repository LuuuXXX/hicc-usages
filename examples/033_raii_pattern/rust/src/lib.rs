//! 自动生成：hicc_usage_raii_pattern
//!
//! 由 tools/rust-gen/rust_gen.py 从 ../ast/symbols.json 生成。
//! 请勿手动编辑；如需调整请修改 tools/rust-gen/special.yaml 后重新生成。

hicc::cpp! {
    #include "hicc_usages/raii_pattern.h"
}
hicc::import_class! {
    #[cpp(class = "hicc_usages::raii_pattern::File", destroy = "hicc_usages::raii_pattern::File::free")]
    pub class File {
        #[cpp(method = "int fd() const")]
        pub fn fd(&self) -> i32;
        #[cpp(method = "bool valid() const")]
        pub fn valid(&self) -> bool;
        #[cpp(method = "int read(int)")]
        pub fn read(&mut self, bytes: i32) -> i32;
        #[cpp(method = "int write(int)")]
        pub fn write(&mut self, bytes: i32) -> i32;
        #[cpp(method = "void close()")]
        pub fn close(&mut self) -> ();
    }
}
hicc::import_lib! {
    #![link_name = "hicc_usage_raii_pattern_adapter"]
    pub class File;
    #[cpp(func = "hicc_usages::raii_pattern::File * hicc_usages::raii_pattern::File::create(int)")]
    pub fn file_new(fd: i32) -> File;
}
