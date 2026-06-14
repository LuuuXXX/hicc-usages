// DIAMOND LIMITATION: full diamond inheritance with virtual base cannot be
// modeled through hicc — vtable offsets and base-pointer adjustments make
// the layout opaque to Rust.
//
// SIMPLIFICATION: collapse the diamond into a single concrete class (Console)
// that combines the middle-tier interfaces. Read the README for context.

hicc::cpp! {
    #include "virtual_diamond.h"
}

hicc::import_class! {
    #[cpp(class = "Console", destroy = "console_free")]
    pub class Console {
        // From Device:
        #[cpp(method = "int priority() const")]
        pub fn priority(&self) -> i32;

        // From InputDevice:
        #[cpp(method = "int read()")]
        pub fn read(&mut self) -> i32;

        // From OutputDevice:
        #[cpp(method = "void write(int)")]
        pub fn write(&mut self, v: i32);
    }
}

hicc::import_lib! {
    #![link_name = "virtual_diamond_hicc"]

    #[cpp(func = "Console* console_new()")]
    pub fn console_new() -> Console;
}
