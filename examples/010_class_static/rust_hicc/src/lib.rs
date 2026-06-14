// Static methods are bound in `import_lib!` as free functions using the
// fully-qualified C++ name (Class::method). Rust sees them as ordinary fns.

hicc::cpp! {
    #include "class_static.h"
}

// We don't need to import the class as a Rust type if Rust only uses static
// methods (no instances cross the FFI boundary). Skip import_class! here.

hicc::import_lib! {
    #![link_name = "class_static_hicc"]

    #[cpp(func = "int Registry::live_count()")]
    pub fn registry_live_count() -> i32;

    #[cpp(func = "int Registry::next_id()")]
    pub fn registry_next_id() -> i32;
}
