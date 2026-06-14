//! 自动生成：hicc_usage_operator_overload
//!
//! 由 tools/rust-gen/rust_gen.py 从 ../ast/symbols.json 生成。
//! 请勿手动编辑；如需调整请修改 tools/rust-gen/special.yaml 后重新生成。

hicc::cpp! {
    #include "hicc_usages/operator_overload.h"

    // ============================================================
    // 自动生成的建议包装（untouched C++ 模式）
    // 取消注释以把对应的 C++ 功能（operator / union 等）暴露到 Rust
    // 不需要就保持注释，对生成的 Rust crate 无影响
    // ============================================================
    // inline hicc_usages::operator_overload::Vec2* vec2_add(const hicc_usages::operator_overload::Vec2& a, const hicc_usages::operator_overload::Vec2& b) { return new hicc_usages::operator_overload::Vec2(a + b); }
    // inline hicc_usages::operator_overload::Vec2* vec2_sub(const hicc_usages::operator_overload::Vec2& a, const hicc_usages::operator_overload::Vec2& b) { return new hicc_usages::operator_overload::Vec2(a - b); }
    // inline hicc_usages::operator_overload::Vec2* vec2_mul(const hicc_usages::operator_overload::Vec2& a, int s) { return new hicc_usages::operator_overload::Vec2(a * s); }
    // inline bool vec2_eq(const hicc_usages::operator_overload::Vec2& a, const hicc_usages::operator_overload::Vec2& b) { return a == b; }
}
hicc::import_class! {
    #[cpp(class = "hicc_usages::operator_overload::Vec2", destroy = "hicc_usages::operator_overload::Vec2::free")]
    pub class Vec2 {
        #[cpp(method = "int get_x() const")]
        pub fn get_x(&self) -> i32;
        #[cpp(method = "int get_y() const")]
        pub fn get_y(&self) -> i32;
        #[cpp(method = "int dot(const hicc_usages::operator_overload::Vec2 &) const")]
        pub fn dot(&self, other: &Vec2) -> i32;
    }
}
hicc::import_lib! {
    #![link_name = "hicc_usage_operator_overload_adapter"]
    pub class Vec2;
    #[cpp(func = "hicc_usages::operator_overload::Vec2 * hicc_usages::operator_overload::Vec2::create(int, int)")]
    pub fn vec2_new(x: i32, y: i32) -> Vec2;
    // #[cpp(func = "hicc_usages::operator_overload::Vec2 * hicc_usages::operator_overload::vec2_add(const hicc_usages::operator_overload::Vec2&, const hicc_usages::operator_overload::Vec2&)")]
    // pub fn vec2_add(a: &Vec2, b: &Vec2) -> Vec2;
    // #[cpp(func = "hicc_usages::operator_overload::Vec2 * hicc_usages::operator_overload::vec2_sub(const hicc_usages::operator_overload::Vec2&, const hicc_usages::operator_overload::Vec2&)")]
    // pub fn vec2_sub(a: &Vec2, b: &Vec2) -> Vec2;
    // #[cpp(func = "hicc_usages::operator_overload::Vec2 * hicc_usages::operator_overload::vec2_mul(const hicc_usages::operator_overload::Vec2&, int)")]
    // pub fn vec2_mul(a: &Vec2, s: i32) -> Vec2;
    // #[cpp(func = "bool hicc_usages::operator_overload::vec2_eq(const hicc_usages::operator_overload::Vec2&, const hicc_usages::operator_overload::Vec2&)")]
    // pub fn vec2_eq(a: &Vec2, b: &Vec2) -> bool;
}
