// Pure virtual: Rust binds the concrete derived class only. The abstract
// base has no factory and is not exposed as a Rust type.

hicc::cpp! {
    #include "virtual_pure.h"

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

hicc::import_class! {
    #[cpp(class = "MemoryStorage", destroy = "mem_storage_free")]
    pub class MemoryStorage {
        #[cpp(method = "std::string get(const std::string&) const")]
        pub fn get(&self, key: &string) -> string;

        #[cpp(method = "void put(const std::string&, const std::string&)")]
        pub fn put(&mut self, key: &string, val: &string);

        #[cpp(method = "int size() const")]
        pub fn size(&self) -> i32;
    }
}

hicc::import_lib! {
    #![link_name = "virtual_pure_hicc"]

    #[cpp(func = "MemoryStorage* mem_storage_new()")]
    pub fn mem_storage_new() -> MemoryStorage;

    #[cpp(func = "std::string* hicc_string_new(const char*)")]
    pub fn string_new(c_str: *const i8) -> string;
}
