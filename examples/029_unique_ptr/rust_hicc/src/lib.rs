// unique_ptr<Widget> return type is transparent to hicc — Rust receives
// an owned Widget, and Drop calls `widget_free` via destroy= attribute.

hicc::cpp! {
    #include "unique_ptr.h"
}

hicc::import_class! {
    #[cpp(class = "Widget", destroy = "widget_free")]
    pub class Widget {
        #[cpp(method = "int value() const")]
        pub fn value(&self) -> i32;
    }
}

hicc::import_lib! {
    #![link_name = "unique_ptr_hicc"]

    // The unique_ptr<Widget> return type is unwrapped to Widget on the Rust side.
    #[cpp(func = "std::unique_ptr<Widget> make_widget(int)")]
    pub fn make_widget(v: i32) -> Widget;
}
