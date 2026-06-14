// Class template path. The hicc::cpp! block typedefs Box<int> and Box<double>
// to concrete C++ types, then exposes factory + deleter as free functions.
// Rust binds the typedef aliases (BoxInt, BoxDouble) as ordinary classes.

hicc::cpp! {
    #include "template_class.h"

    using BoxInt    = BoxT<int>;
    using BoxDouble = BoxT<double>;

    inline BoxInt*    box_int_new(int v)         { return new BoxInt(v); }
    inline void       box_int_free(BoxInt* b)    { delete b; }
    inline BoxDouble* box_double_new(double v)   { return new BoxDouble(v); }
    inline void       box_double_free(BoxDouble* b) { delete b; }
}

hicc::import_class! {
    #[cpp(class = "BoxInt", destroy = "box_int_free")]
    pub class BoxInt {
        #[cpp(method = "int get() const")]
        pub fn get(&self) -> i32;
        #[cpp(method = "void set(int)")]
        pub fn set(&mut self, v: i32);
    }
}

hicc::import_class! {
    #[cpp(class = "BoxDouble", destroy = "box_double_free")]
    pub class BoxDouble {
        #[cpp(method = "double get() const")]
        pub fn get(&self) -> f64;
        #[cpp(method = "void set(double)")]
        pub fn set(&mut self, v: f64);
    }
}

hicc::import_lib! {
    #![link_name = "template_class_hicc"]

    #[cpp(func = "BoxInt* box_int_new(int)")]
    pub fn box_int_new(v: i32) -> BoxInt;

    #[cpp(func = "BoxDouble* box_double_new(double)")]
    pub fn box_double_new(v: f64) -> BoxDouble;
}
