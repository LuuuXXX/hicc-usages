// ⚠️ KEY HICC PATTERN: std::string MUST be bound via import_class! — do NOT
// use hicc_std::string alias (memory layout incompatible → segfault).

hicc::cpp! {
    #include "string_basic.h"

    inline std::string* hicc_string_new(const char* s) { return new std::string(s); }
    inline void         hicc_string_free(std::string* s) { delete s; }
}

hicc::import_class! {
    #[cpp(class = "std::string", destroy = "hicc_string_free")]
    pub class string {
        #[cpp(method = "const char* c_str() const")]
        pub fn c_str(&self) -> *const i8;
    }
}

hicc::import_lib! {
    #![link_name = "string_basic_hicc"]

    #[cpp(func = "std::string concat(const std::string&, const std::string&)")]
    pub fn concat(a: &string, b: &string) -> string;

    #[cpp(func = "std::string upper(const std::string&)")]
    pub fn upper(s: &string) -> string;

    #[cpp(func = "std::size_t length(const std::string&)")]
    pub fn length(s: &string) -> usize;

    #[cpp(func = "std::string* hicc_string_new(const char*)")]
    pub fn string_new(c_str: *const i8) -> string;
}
