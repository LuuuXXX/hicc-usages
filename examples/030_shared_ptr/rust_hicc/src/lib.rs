// shared_ptr<T> return: stripped to owned T. (Note: the shared_ptr's
// refcount is bypassed by this binding — we treat it as unique ownership.
// For true shared semantics across Rust instances, the C++ wrapper would
// need to return shared_ptr copies; that's beyond this demo.)

hicc::cpp! {
    #include "shared_ptr.h"
}

hicc::import_class! {
    #[cpp(class = "RefCounted", destroy = "shared_obj_free")]
    pub class RefCounted {
        #[cpp(method = "int use_count() const")]
        pub fn use_count(&self) -> i32;
    }
}

hicc::import_lib! {
    #![link_name = "shared_ptr_hicc"]

    #[cpp(func = "std::shared_ptr<RefCounted> make_shared_obj()")]
    pub fn make_shared_obj() -> RefCounted;

    #[cpp(func = "int shared_count()")]
    pub fn shared_count() -> i32;
}
