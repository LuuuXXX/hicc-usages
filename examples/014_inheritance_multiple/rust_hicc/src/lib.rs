// Multiple inheritance: Sprite derives from Drawable and Serializable.
// Rust flattens — Sprite is a single Rust struct exposing all methods
// (own + both bases).

hicc::cpp! {
    #include "inheritance_multiple.h"
}

hicc::import_class! {
    #[cpp(class = "Sprite", destroy = "sprite_free")]
    pub class Sprite {
        // From Drawable:
        #[cpp(method = "void draw() const")]
        pub fn draw(&self);

        // From Serializable:
        #[cpp(method = "int byte_size() const")]
        pub fn byte_size(&self) -> i32;

        // Own:
        #[cpp(method = "int width() const")]
        pub fn width(&self) -> i32;

        #[cpp(method = "int height() const")]
        pub fn height(&self) -> i32;
    }
}

hicc::import_lib! {
    #![link_name = "inheritance_multiple_hicc"]

    #[cpp(func = "Sprite* sprite_new(int, int)")]
    pub fn sprite_new(w: i32, h: i32) -> Sprite;
}
