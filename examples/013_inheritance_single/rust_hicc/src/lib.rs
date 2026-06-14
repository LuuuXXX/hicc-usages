// Single inheritance: Square derives from Shape. Rust treats Square as an
// independent class. Methods from the base (id()) are re-declared here —
// they're compiled to the same vtable entry on the C++ side.

hicc::cpp! {
    #include "inheritance_single.h"
}

hicc::import_class! {
    #[cpp(class = "Square", destroy = "square_free")]
    pub class Square {
        // Derived-class own methods:
        #[cpp(method = "int area() const")]
        pub fn area(&self) -> i32;

        #[cpp(method = "int side() const")]
        pub fn side(&self) -> i32;

        // Base-class method re-declared on the derived type:
        #[cpp(method = "int id() const")]
        pub fn id(&self) -> i32;
    }
}

hicc::import_lib! {
    #![link_name = "inheritance_single_hicc"]

    #[cpp(func = "Square* square_new(int)")]
    pub fn square_new(side: i32) -> Square;
}
