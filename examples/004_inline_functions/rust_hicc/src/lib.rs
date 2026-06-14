// Inline functions are transparent to hicc — bind as ordinary functions.

hicc::cpp! {
    #include "inline_functions.h"
}

hicc::import_lib! {
    #![link_name = "inline_functions_hicc"]

    #[cpp(func = "int square(int)")]
    pub fn square(x: i32) -> i32;

    #[cpp(func = "int cube(int)")]
    pub fn cube(x: i32) -> i32;
}
