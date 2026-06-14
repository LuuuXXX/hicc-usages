// Synthesis: combines class+factory/free, Exception<T>, int-bridge for enum,
// and std::string→const char* adapter at the boundary.

hicc::cpp! {
    #include "summary.h"
}

hicc::import_class! {
    #[cpp(class = "Calculator", destroy = "calc_free")]
    pub class Calculator {
        #[cpp(method = "int base() const")]
        pub fn base(&self) -> i32;

        #[cpp(method = "int apply(OpKind, int, int) const")]
        pub fn apply(&self, op: i32, x: i32, y: i32) -> hicc::Exception<i32>;
    }
}

hicc::import_lib! {
    #![link_name = "summary_hicc"]

    #[cpp(func = "Calculator* calc_new(int)")]
    pub fn calc_new(seed: i32) -> Calculator;

    #[cpp(func = "int op_kind_int(int)")]
    pub fn op_kind_int(kind: i32) -> i32;

    #[cpp(func = "const char* calc_version()")]
    pub fn calc_version() -> *const i8;

    #[cpp(func = "const char* calc_describe_c(const Calculator*)")]
    pub fn describe(c: &Calculator) -> *const i8;
}
