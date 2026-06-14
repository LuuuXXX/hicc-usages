//! 自动生成：hicc_usage_string_basic
//!
//! 由 tools/rust-gen/rust_gen.py 从 ../ast/symbols.json 生成。
//! 请勿手动编辑；如需调整请修改 tools/rust-gen/special.yaml 后重新生成。

hicc::cpp! {
    #include "hicc_usages/string_basic.h"
}
hicc::import_class! {
    #[cpp(class = "hicc_usages::string_basic::StringBuf", destroy = "hicc_usages::string_basic::StringBuf::free")]
    pub class StringBuf {
        #[cpp(method = "void append(const char *)")]
        pub fn append(&mut self, s: *const i8) -> ();
        #[cpp(method = "const char * c_str() const")]
        pub fn c_str(&self) -> *const i8;
        #[cpp(method = "bool equals(const char *) const")]
        pub fn equals(&self, s: *const i8) -> bool;
        #[cpp(method = "int find(const char *) const")]
        pub fn find(&self, needle: *const i8) -> i32;
        #[cpp(method = "const char * substring(std::size_t, std::size_t) const")]
        pub fn substring(&self, start: usize, len: usize) -> *const i8;
    }
}
hicc::import_lib! {
    #![link_name = "hicc_usage_string_basic_adapter"]
    pub class StringBuf;
    #[cpp(func = "hicc_usages::string_basic::StringBuf * hicc_usages::string_basic::StringBuf::create()")]
    pub fn stringbuf_new() -> StringBuf;
    #[cpp(func = "hicc_usages::string_basic::StringBuf * hicc_usages::string_basic::StringBuf::create_from(const char *)")]
    pub fn stringbuf_from(s: *const i8) -> StringBuf;
}
