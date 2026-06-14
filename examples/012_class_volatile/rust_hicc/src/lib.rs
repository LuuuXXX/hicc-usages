// Volatile methods use the `volatile` qualifier in `#[cpp(method = ...)]`.
// hicc maps them to `&mut self` (volatile implies mutation is observable).

hicc::cpp! {
    #include "class_volatile.h"
}

hicc::import_class! {
    #[cpp(class = "VCounter", destroy = "vcounter_free")]
    pub class VCounter {
        #[cpp(method = "void inc() volatile")]
        pub fn inc(&mut self);

        #[cpp(method = "int get() const volatile")]
        pub fn get(&self) -> i32;

        #[cpp(method = "void reset() volatile")]
        pub fn reset(&mut self);
    }
}

hicc::import_lib! {
    #![link_name = "class_volatile_hicc"]

    #[cpp(func = "VCounter* vcounter_new()")]
    pub fn vcounter_new() -> VCounter;
}
