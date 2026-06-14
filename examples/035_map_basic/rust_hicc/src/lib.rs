// StringIntMap wraps std::map<std::string, int>. Expose accessor methods
// that take/return std::string (which we bind separately as a class).

hicc::cpp! {
    #include "map_basic.h"

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
    #[cpp(class = "StringIntMap", destroy = "str_int_map_free")]
    pub class StringIntMap {
        #[cpp(method = "void insert(const std::string&, int)")]
        pub fn insert(&mut self, k: &string, v: i32);

        #[cpp(method = "std::size_t size() const")]
        pub fn size(&self) -> usize;

        #[cpp(method = "bool contains(const std::string&) const")]
        pub fn contains(&self, k: &string) -> bool;

        #[cpp(method = "int get_or(const std::string&, int) const")]
        pub fn get_or(&self, k: &string, def: i32) -> i32;
    }
}

hicc::import_lib! {
    #![link_name = "map_basic_hicc"]

    #[cpp(func = "StringIntMap* str_int_map_new()")]
    pub fn str_int_map_new() -> StringIntMap;

    #[cpp(func = "std::string* hicc_string_new(const char*)")]
    pub fn string_new(c_str: *const i8) -> string;
}
