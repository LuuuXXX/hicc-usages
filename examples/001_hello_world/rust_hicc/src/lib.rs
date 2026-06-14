// Bind the externally-built C++ library (../cpp/build/libhello_world.a).
// hicc::cpp! block includes the public header and provides tiny free
// functions for std::string lifecycle (can't add static methods to a
// standard type, so free functions are wired via the `destroy` attribute).

hicc::cpp! {
    #include "hello_world.h"

    inline std::string* hicc_string_new(const char* s) { return new std::string(s); }
    inline void hicc_string_free(std::string* s) { delete s; }
}

// std::string binding — must use import_class!, NOT hicc_std::string (see
// docs/hicc-capabilities.md §4.6). Constructors are exposed as free factory
// functions in import_lib! below (hicc convention).
hicc::import_class! {
    #[cpp(class = "std::string", destroy = "hicc_string_free")]
    pub class string {
        #[cpp(method = "const char* c_str() const")]
        pub fn c_str(&self) -> *const i8;
    }
}

hicc::import_lib! {
    #![link_name = "hello_world_hicc"]

    // std::string* hicc_string_new(const char*) — factory for std::string.
    #[cpp(func = "std::string* hicc_string_new(const char*)")]
    pub fn string_new(c_str: *const i8) -> string;

    // std::string hello(const std::string& who)
    #[cpp(func = "std::string hello(const std::string&)")]
    pub fn hello(who: &string) -> string;

    // int add(int, int)
    #[cpp(func = "int add(int, int)")]
    pub fn add(a: i32, b: i32) -> i32;
}
