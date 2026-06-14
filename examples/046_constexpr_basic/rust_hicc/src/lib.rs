// constexpr is transparent: hicc binds signatures, the constexpr-ness is
// invisible across FFI. sq/cube are inline in the header; we also expose
// square_plus_magic from the .cpp so the linker has real work.

hicc::cpp! {
    #include "constexpr_basic.h"
}

hicc::import_lib! {
    #![link_name = "constexpr_basic_hicc"]

    #[cpp(func = "int sq(int)")]
    pub fn sq(x: i32) -> i32;

    #[cpp(func = "int cube(int)")]
    pub fn cube(x: i32) -> i32;

    #[cpp(func = "int square_plus_magic(int)")]
    pub fn square_plus_magic(x: i32) -> i32;
}
