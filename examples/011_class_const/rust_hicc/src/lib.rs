// Vec2: all methods are const → Rust `&self`.

hicc::cpp! {
    #include "class_const.h"

    inline Vec2* vec2_new(double x, double y) { return new Vec2(x, y); }
    inline void  vec2_free(Vec2* v)           { delete v; }
}

hicc::import_class! {
    #[cpp(class = "Vec2", destroy = "vec2_free")]
    pub class Vec2 {
        #[cpp(method = "double x() const")]
        pub fn x(&self) -> f64;

        #[cpp(method = "double y() const")]
        pub fn y(&self) -> f64;

        #[cpp(method = "double magnitude() const")]
        pub fn magnitude(&self) -> f64;

        #[cpp(method = "double dot(const Vec2&) const")]
        pub fn dot(&self, other: &Vec2) -> f64;
    }
}

hicc::import_lib! {
    #![link_name = "class_const_hicc"]

    #[cpp(func = "Vec2* vec2_new(double, double)")]
    pub fn vec2_new(x: f64, y: f64) -> Vec2;
}
