// Copy ctor is exposed via `box_clone(&src) -> Box`. Rust's Clone trait can be
// implemented on top of it.

hicc::cpp! {
    #include "class_copy.h"
}

hicc::import_class! {
    #[cpp(class = "Box", destroy = "box_free")]
    pub class Box {
        #[cpp(method = "int get() const")]
        pub fn get(&self) -> i32;

        #[cpp(method = "void set(int)")]
        pub fn set(&mut self, v: i32);
    }
}

hicc::import_lib! {
    #![link_name = "class_copy_hicc"]

    #[cpp(func = "Box* box_new(int)")]
    pub fn box_new(v: i32) -> Box;

    #[cpp(func = "Box* box_clone(const Box*)")]
    pub fn box_clone(src: &Box) -> Box;
}
