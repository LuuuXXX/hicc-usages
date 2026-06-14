// Specialization: Rust can't name the specialized C++ class directly, so
// we wrap each specialization's static method in a free function on the C++
// side and bind those.

hicc::cpp! {
    #include "template_specialization.h"
}

hicc::import_lib! {
    #![link_name = "template_specialization_hicc"]

    #[cpp(func = "const char* type_name_int()")]
    pub fn type_name_int() -> *const i8;

    #[cpp(func = "const char* type_name_bool()")]
    pub fn type_name_bool() -> *const i8;

    #[cpp(func = "const char* type_name_generic()")]
    pub fn type_name_generic() -> *const i8;
}
