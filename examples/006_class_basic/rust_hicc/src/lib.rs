// Counter: basic class. `destroy = "counter_free"` wires the C++ deleter into
// Rust's Drop path so `let _c = counter_new();` won't leak.

hicc::cpp! {
    #include "class_basic.h"
}

hicc::import_class! {
    #[cpp(class = "Counter", destroy = "counter_free")]
    pub class Counter {
        #[cpp(method = "int get() const")]
        pub fn get(&self) -> i32;

        #[cpp(method = "void inc()")]
        pub fn inc(&mut self);

        #[cpp(method = "void reset()")]
        pub fn reset(&mut self);
    }
}

hicc::import_lib! {
    #![link_name = "class_basic_hicc"]

    #[cpp(func = "Counter* counter_new()")]
    pub fn counter_new() -> Counter;
}
