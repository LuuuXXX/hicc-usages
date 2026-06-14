// Point: parameterized constructor exposed via factory `point_new(x, y)`.

hicc::cpp! {
    #include "class_constructor.h"
}

hicc::import_class! {
    #[cpp(class = "Point", destroy = "point_free")]
    pub class Point {
        #[cpp(method = "int get_x() const")]
        pub fn get_x(&self) -> i32;

        #[cpp(method = "int get_y() const")]
        pub fn get_y(&self) -> i32;

        #[cpp(method = "int manhattan() const")]
        pub fn manhattan(&self) -> i32;
    }
}

hicc::import_lib! {
    #![link_name = "class_constructor_hicc"]

    #[cpp(func = "Point* point_new(int, int)")]
    pub fn point_new(x: i32, y: i32) -> Point;
}
