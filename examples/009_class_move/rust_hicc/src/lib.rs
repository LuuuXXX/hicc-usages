// The C++ method `int consume_value() &&` (rvalue-qualified) is exposed in
// Rust as `fn consume_value(self) -> i32`. Rust taking `self` by value means
// the receiver is consumed; hicc translates that to the && call.

hicc::cpp! {
    #include "class_move.h"
}

hicc::import_class! {
    #[cpp(class = "Resource", destroy = "resource_free")]
    pub class Resource {
        #[cpp(method = "int consume_value() &&")]
        pub fn consume_value(self) -> i32;

        #[cpp(method = "int peek() const")]
        pub fn peek(&self) -> i32;

        #[cpp(method = "bool is_valid() const")]
        pub fn is_valid(&self) -> bool;
    }
}

hicc::import_lib! {
    #![link_name = "class_move_hicc"]

    #[cpp(func = "Resource* resource_new(int)")]
    pub fn resource_new(v: i32) -> Resource;
}
