// std::array<int, 4> wrapped behind a typedef alias (same pattern as class
// templates). Rust binds the alias as a normal class.

hicc::cpp! {
    #include "array_basic.h"
}

hicc::import_class! {
    #[cpp(class = "IntArray4", destroy = "int_array4_free")]
    pub class IntArray4 {
        #[cpp(method = "void set(std::size_t, int)")]
        pub fn set(&mut self, i: usize, v: i32);

        #[cpp(method = "int get(std::size_t) const")]
        pub fn get(&self, i: usize) -> i32;

        #[cpp(method = "std::size_t size() const")]
        pub fn size(&self) -> usize;

        #[cpp(method = "int sum() const")]
        pub fn sum(&self) -> i32;
    }
}

hicc::import_lib! {
    #![link_name = "array_basic_hicc"]

    #[cpp(func = "IntArray4* int_array4_new()")]
    pub fn int_array4_new() -> IntArray4;
}
