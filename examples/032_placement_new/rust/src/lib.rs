//! 自动生成：hicc_usage_placement_new
//!
//! 由 tools/rust-gen/rust_gen.py 从 ../ast/symbols.json 生成。
//! 请勿手动编辑；如需调整请修改 tools/rust-gen/special.yaml 后重新生成。

hicc::cpp! {
    #include "hicc_usages/placement_new.h"
}
hicc::import_class! {
    #[cpp(class = "hicc_usages::placement_new::Buffer", destroy = "hicc_usages::placement_new::Buffer::free")]
    pub class Buffer {
        #[cpp(method = "void reset()")]
        pub fn reset(&mut self) -> ();
        #[cpp(method = "int place_int(int)")]
        pub fn place_int(&mut self, v: i32) -> i32;
        #[cpp(method = "int get_int(std::size_t) const")]
        pub fn get_int(&self, idx: usize) -> i32;
        #[cpp(method = "double place_double(double)")]
        pub fn place_double(&mut self, v: f64) -> f64;
        #[cpp(method = "double get_double(std::size_t) const")]
        pub fn get_double(&self, idx: usize) -> f64;
    }
}
hicc::import_lib! {
    #![link_name = "hicc_usage_placement_new_adapter"]
    pub class Buffer;
    #[cpp(func = "hicc_usages::placement_new::Buffer * hicc_usages::placement_new::Buffer::create(std::size_t)")]
    pub fn buffer_new(capacity: usize) -> Buffer;
}
