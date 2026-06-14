//! 048_summary: combined showcase
//!
//! 混合 5 种模式（来自 042/043/044/046/015）：
//! 1. 类 `Customer`（构造 + std::string 字段 + std::vector 成员 + Exception 路径）
//! 2. enum class `CustomerTier`（int<->enum 转换器 + Rust 镜像）
//! 3. 单继承 + virtual method（仅绑派生类 VipCustomer）
//! 4. 静态 constexpr 数据成员（Settings::MAX_CUSTOMERS / DEFAULT_DISCOUNT）
//! 5. std::vector 参数/返回（doubled_values 自由函数）

hicc::cpp! {
    #include "summary.h"
    #include <hicc/std/string.hpp>
    #include <hicc/std/vector.hpp>

    typedef std::vector<int> CppVec;

    // Inline helpers to bridge enum class and CustomerBase& to FFI-friendly forms.
    inline int customer_tier_int(const summary_ns::Customer& c) {
        return static_cast<int>(c.tier());
    }

    inline double discounted_price_vip(const summary_ns::VipCustomer& c, double price) {
        return summary_ns::compute_discounted_price(c, price);
    }
}

/// Rust-side mirror of `CustomerTier` enum class.
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CustomerTier { Free = 0, Basic = 1, Premium = 2 }

hicc::import_class! {
    class string = hicc_std::string;

    #[cpp(class = "summary_ns::Customer")]
    pub class Customer {
        #[cpp(method = "int id() const")]
        pub fn id(&self) -> i32;

        #[cpp(method = "std::string name() const")]
        pub fn name(&self) -> string;

        // tier() returns CustomerTier in C++ (enum class) — wrap to int via inline
        pub fn tier(&self) -> CustomerTier {
            let v: i32 = customer_tier_int(self);
            match v {
                0 => CustomerTier::Free,
                1 => CustomerTier::Basic,
                _ => CustomerTier::Premium,
            }
        }

        #[cpp(method = "void rename(const std::string&)")]
        pub fn rename(&mut self, new_name: &string);

        #[cpp(method = "void upgrade(summary_ns::CustomerTier)")]
        pub fn upgrade(&mut self, new_tier: i32) -> hicc::Exception<()>;

        #[cpp(method = "void charge(int)")]
        pub fn charge(&mut self, amount: i32) -> hicc::Exception<()>;

        #[cpp(method = "int purchase_at(size_t) const")]
        pub fn purchase_at(&self, idx: usize) -> hicc::Exception<i32>;

        #[cpp(method = "size_t purchase_count() const")]
        pub fn purchase_count(&self) -> usize;

        #[cpp(method = "int total_spent() const")]
        pub fn total_spent(&self) -> i32;

        #[cpp(method = "void add_purchase(int)")]
        pub fn add_purchase(&mut self, amount: i32);

        #[cpp(method = "std::string describe() const")]
        pub fn describe(&self) -> string;

        pub fn new(id: i32, name: &string, tier: CustomerTier) -> Self {
            make_customer(id, name, tier as i32)
        }
    }

    #[cpp(class = "summary_ns::VipCustomer")]
    pub class VipCustomer {
        #[cpp(method = "std::string label() const")]
        pub fn label(&self) -> string;

        #[cpp(method = "double discount() const")]
        pub fn discount(&self) -> f64;

        pub fn new(discount_rate: f64) -> Self { make_vip(discount_rate) }
    }
}

hicc::import_lib! {
    #![link_name = "summary"]

    class RustVec = hicc_std::vector<hicc::Pod<i32>>;

    // Empty vector factory (for doubled_values demo)
    #[cpp(func = "std::unique_ptr<CppVec> hicc::make_unique<CppVec>()")]
    pub fn vec_new() -> RustVec;

    #[cpp(func = "std::unique_ptr<summary_ns::Customer> summary_ns::make_customer(int, const std::string&, int)")]
    pub fn make_customer(id: i32, name: &string, tier: i32) -> Customer;

    #[cpp(func = "std::unique_ptr<summary_ns::VipCustomer> summary_ns::make_vip(double)")]
    pub fn make_vip(discount_rate: f64) -> VipCustomer;

    // compute_discounted_price takes CustomerBase& — wrap via inline helper for VipCustomer
    #[cpp(func = "double discounted_price_vip(const summary_ns::VipCustomer&, double)")]
    pub fn compute_discounted_price(c: &VipCustomer, price: f64) -> f64;

    // std::vector<int> free function (from 034 pattern)
    #[cpp(func = "std::vector<int> summary_ns::doubled_values(const std::vector<int>&)")]
    pub fn doubled_values(v: &RustVec) -> RustVec;

    // Static constexpr data (from 046 pattern)
    #[cpp(data = "summary_ns::Settings::MAX_CUSTOMERS")]
    pub fn max_customers() -> &'static i32;

    #[cpp(data = "summary_ns::Settings::DEFAULT_DISCOUNT")]
    pub fn default_discount() -> &'static f64;

    // inline helper that returns Customer::tier() as int
    #[cpp(func = "int customer_tier_int(const summary_ns::Customer&)")]
    pub fn customer_tier_int(c: &Customer) -> i32;
}
