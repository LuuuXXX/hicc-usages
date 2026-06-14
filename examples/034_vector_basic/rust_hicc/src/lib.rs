// IntVector wraps std::vector<int>. We expose push/size/at as ordinary
// methods — no hicc-std alias needed since we don't return the raw vector.

hicc::cpp! {
    #include "vector_basic.h"
}

hicc::import_class! {
    #[cpp(class = "IntVector", destroy = "int_vec_free")]
    pub class IntVector {
        #[cpp(method = "void push(int)")]
        pub fn push(&mut self, v: i32);

        #[cpp(method = "std::size_t size() const")]
        pub fn size(&self) -> usize;

        #[cpp(method = "int at(std::size_t) const")]
        pub fn at(&self, i: usize) -> i32;
    }
}

hicc::import_lib! {
    #![link_name = "vector_basic_hicc"]

    #[cpp(func = "IntVector* int_vec_new()")]
    pub fn int_vec_new() -> IntVector;
}
