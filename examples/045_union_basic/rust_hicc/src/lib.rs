// Union itself is not nameable across FFI. ValueBox wraps the union and
// exposes typed setters/getters; Rust sees only the box + tag int.

hicc::cpp! {
    #include "union_basic.h"
}

hicc::import_class! {
    #[cpp(class = "ValueBox", destroy = "value_box_free")]
    pub class ValueBox {
        #[cpp(method = "void set_int(int)")]
        pub fn set_int(&mut self, v: i32);

        #[cpp(method = "void set_float(float)")]
        pub fn set_float(&mut self, v: f32);

        #[cpp(method = "int get_int() const")]
        pub fn get_int(&self) -> i32;

        #[cpp(method = "float get_float() const")]
        pub fn get_float(&self) -> f32;

        #[cpp(method = "int tag() const")]
        pub fn tag(&self) -> i32;
    }
}

hicc::import_lib! {
    #![link_name = "union_basic_hicc"]

    #[cpp(func = "ValueBox* value_box_new()")]
    pub fn value_box_new() -> ValueBox;
}
