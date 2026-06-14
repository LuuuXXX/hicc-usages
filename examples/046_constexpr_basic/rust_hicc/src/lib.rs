//! 046_constexpr_basic: constexpr static data + constexpr methods
//!
//! hicc 模式：
//! 1. **静态 constexpr 数据成员**：用 `#[cpp(data = "ns::Class::FIELD")]` 暴露，
//!    返回 `&'static T`。hicc 用 `EXPORT_CONST_DATA` 包装。
//! 2. **constexpr 自由函数**：完全透明 —— 编译期/运行期两用，FFI 时当作普通函数。
//! 3. **constexpr 方法**：与普通 `const` 方法一样绑定，hicc-build 不感知 `constexpr`。
//!
//! 关键点：`constexpr` 是编译期提示，对运行时 ABI 完全无影响。

hicc::cpp! {
    #include "constexpr_basic.h"
}

hicc::import_class! {
    #[cpp(class = "constexpr_basic_ns::Circle")]
    pub class Circle {
        #[cpp(method = "double radius() const")]
        pub fn radius(&self) -> f64;

        #[cpp(method = "double area() const")]
        pub fn area(&self) -> f64;

        #[cpp(method = "void set_radius(double)")]
        pub fn set_radius(&mut self, r: f64);
    }
}

hicc::import_lib! {
    #![link_name = "constexpr_basic"]

    // Static constexpr data exposed as &'static references
    #[cpp(data = "constexpr_basic_ns::Constants::PI")]
    pub fn pi() -> &'static f64;

    #[cpp(data = "constexpr_basic_ns::Constants::E")]
    pub fn e_constant() -> &'static f64;

    #[cpp(data = "constexpr_basic_ns::Constants::BUFFER_SIZE")]
    pub fn buffer_size() -> &'static i32;

    #[cpp(data = "constexpr_basic_ns::Constants::MAX_TRIES")]
    pub fn max_tries() -> &'static i32;

    #[cpp(data = "constexpr_basic_ns::Constants::BIG_NUMBER")]
    pub fn big_number() -> &'static i64;

    // constexpr free functions — transparent
    #[cpp(func = "int constexpr_basic_ns::square(int)")]
    pub fn square(x: i32) -> i32;

    #[cpp(func = "long constexpr_basic_ns::factorial(int)")]
    pub fn factorial(n: i32) -> i64;

    #[cpp(func = "double constexpr_basic_ns::compute_area(double)")]
    pub fn compute_area(radius: f64) -> f64;
}
