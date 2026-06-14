// ⚠️ hicc's #[cpp(method = "...")] does NOT accept `noexcept` in the
// signature. C++ side adjusted: dropped `noexcept` from member methods.
// The semantic (no-throw) is still enforced at runtime by C++.

hicc::cpp! {
    #include "noexcept_basic.h"
}

hicc::import_class! {
    #[cpp(class = "SafeAdder", destroy = "safe_adder_free")]
    pub class SafeAdder {
        #[cpp(method = "int add(int) const")]
        pub fn add(&self, x: i32) -> i32;

        #[cpp(method = "int sub(int) const")]
        pub fn sub(&self, x: i32) -> i32;

        #[cpp(method = "int combined(int, int) const")]
        pub fn combined(&self, x: i32, y: i32) -> i32;
    }
}

hicc::import_lib! {
    #![link_name = "noexcept_basic_hicc"]

    #[cpp(func = "SafeAdder* safe_adder_new(int)")]
    pub fn safe_adder_new(base: i32) -> SafeAdder;
}
