//! 048_summary: 综合展示
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

    // inline 辅助函数：将 enum class 与 CustomerBase& 桥接为 FFI 友好形式。
    inline int customer_tier_int(const summary_ns::Customer& c) {
        return static_cast<int>(c.tier());
    }

    inline double discounted_price_vip(const summary_ns::VipCustomer& c, double price) {
        return summary_ns::compute_discounted_price(c, price);
    }
}

/// `CustomerTier` enum class 的 Rust 端镜像。
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

        // tier() 在 C++ 中返回 CustomerTier（enum class）—— 通过 inline 包装为 int
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

    // 空 vector 工厂（用于 doubled_values 演示）
    #[cpp(func = "std::unique_ptr<CppVec> hicc::make_unique<CppVec>()")]
    pub fn vec_new() -> RustVec;

    #[cpp(func = "std::unique_ptr<summary_ns::Customer> summary_ns::make_customer(int, const std::string&, int)")]
    pub fn make_customer(id: i32, name: &string, tier: i32) -> Customer;

    #[cpp(func = "std::unique_ptr<summary_ns::VipCustomer> summary_ns::make_vip(double)")]
    pub fn make_vip(discount_rate: f64) -> VipCustomer;

    // compute_discounted_price 接收 CustomerBase& —— 对 VipCustomer 通过 inline 辅助函数包装
    #[cpp(func = "double discounted_price_vip(const summary_ns::VipCustomer&, double)")]
    pub fn compute_discounted_price(c: &VipCustomer, price: f64) -> f64;

    // std::vector<int> 自由函数（来自 034 模式）
    #[cpp(func = "std::vector<int> summary_ns::doubled_values(const std::vector<int>&)")]
    pub fn doubled_values(v: &RustVec) -> RustVec;

    // 静态 constexpr 数据（来自 046 模式）
    #[cpp(data = "summary_ns::Settings::MAX_CUSTOMERS")]
    pub fn max_customers() -> &'static i32;

    #[cpp(data = "summary_ns::Settings::DEFAULT_DISCOUNT")]
    pub fn default_discount() -> &'static f64;

    // inline 辅助函数：将 Customer::tier() 以 int 返回
    #[cpp(func = "int customer_tier_int(const summary_ns::Customer&)")]
    pub fn customer_tier_int(c: &Customer) -> i32;
}
