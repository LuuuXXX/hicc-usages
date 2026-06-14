// ⚠️ LIMITATION: std::tuple can't be named at FFI boundary. We wrap it in a
// class (Triple) with named accessors first/second/third.

hicc::cpp! {
    #include "tuple_basic.h"

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
    #[cpp(class = "Triple", destroy = "triple_free")]
    pub class Triple {
        #[cpp(method = "int first() const")]
        pub fn first(&self) -> i32;

        #[cpp(method = "std::string second() const")]
        pub fn second(&self) -> string;

        #[cpp(method = "double third() const")]
        pub fn third(&self) -> f64;
    }
}

hicc::import_lib! {
    #![link_name = "tuple_basic_hicc"]

    #[cpp(func = "Triple* triple_new(int, const char*, double)")]
    pub fn triple_new(i: i32, s: *const i8, d: f64) -> Triple;
}
