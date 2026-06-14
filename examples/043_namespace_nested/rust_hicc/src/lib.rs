// Nested namespaces are transparent: hicc binds fully-qualified signatures.
// We only expose top-level wrappers (ns_add/ns_mul/ns_combined); the
// inner namespaces stay internal to C++.

hicc::cpp! {
    #include "namespace_nested.h"
}

hicc::import_lib! {
    #![link_name = "namespace_nested_hicc"]

    #[cpp(func = "int ns_add(int, int)")]
    pub fn ns_add(a: i32, b: i32) -> i32;

    #[cpp(func = "int ns_mul(int, int)")]
    pub fn ns_mul(a: i32, b: i32) -> i32;

    #[cpp(func = "int ns_combined(int, int, int)")]
    pub fn ns_combined(a: i32, b: i32, c: i32) -> i32;
}
