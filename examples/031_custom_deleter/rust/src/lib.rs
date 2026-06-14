//! 自动生成：hicc_usage_custom_deleter
//!
//! 由 tools/rust-gen/rust_gen.py 从 ../ast/symbols.json 生成。
//! 请勿手动编辑；如需调整请修改 tools/rust-gen/special.yaml 后重新生成。

hicc::cpp! {
    #include "hicc_usages/custom_deleter.h"
}
hicc::import_class! {
    #[cpp(class = "hicc_usages::custom_deleter::FileHandle", destroy = "hicc_usages::custom_deleter::FileHandle::free")]
    pub class FileHandle {
        #[cpp(method = "int fd() const")]
        pub fn fd(&self) -> i32;
        #[cpp(method = "bool closed() const")]
        pub fn closed(&self) -> bool;
        #[cpp(method = "void close()")]
        pub fn close(&mut self) -> ();
    }
}
hicc::import_class! {
    #[cpp(class = "hicc_usages::custom_deleter::FileManager", destroy = "hicc_usages::custom_deleter::FileManager::free")]
    pub class FileManager {
        #[cpp(method = "void open(int)")]
        pub fn open(&mut self, fd: i32) -> ();
        #[cpp(method = "bool close(int)")]
        pub fn close(&mut self, fd: i32) -> bool;
    }
}
hicc::import_lib! {
    #![link_name = "hicc_usage_custom_deleter_adapter"]
    pub class FileHandle;
    pub class FileManager;
    #[cpp(func = "hicc_usages::custom_deleter::FileHandle * hicc_usages::custom_deleter::FileHandle::create(int)")]
    pub fn filehandle_new(fd: i32) -> FileHandle;
    #[cpp(func = "hicc_usages::custom_deleter::FileManager * hicc_usages::custom_deleter::FileManager::create()")]
    pub fn filemanager_new() -> FileManager;
}
