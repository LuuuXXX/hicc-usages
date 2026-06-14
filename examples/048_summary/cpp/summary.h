#pragma once
#include <string>
#include <vector>
#include <memory>
#include <stdexcept>
#include <iostream>

namespace summary_ns {

// ===== 模式 1：enum class 经由 int 桥接（来自 044） =====
enum class CustomerTier : int { Free = 0, Basic = 1, Premium = 2 };

inline int tier_to_int(CustomerTier t) { return static_cast<int>(t); }
inline CustomerTier tier_from_int(int v) {
    if (v < 0 || v > 2) return CustomerTier::Free;
    return static_cast<CustomerTier>(v);
}

// ===== 模式 2：带 std::string 字段、std::vector、错误时抛异常的类（综合 006/034/042） =====
class Customer {
public:
    Customer(int id, const std::string& name, CustomerTier tier);

    int id() const;
    std::string name() const;
    CustomerTier tier() const;

    void rename(const std::string& new_name);
    void upgrade(CustomerTier new_tier);

    // amount <= 0 时抛 std::invalid_argument
    void charge(int amount);

    // idx >= purchases.size() 时抛 std::out_of_range
    int purchase_at(size_t idx) const;

    size_t purchase_count() const;
    int total_spent() const;

    void add_purchase(int amount);

    std::string describe() const;

private:
    int id_;
    std::string name_;
    CustomerTier tier_;
    std::vector<int> purchases_;
    int total_spent_;
};

std::unique_ptr<Customer> make_customer(int id, const std::string& name, int tier);

// ===== 模式 3：单继承虚函数（来自 013/015） =====
class CustomerBase {
public:
    CustomerBase() = default;
    virtual ~CustomerBase() = default;
    virtual std::string label() const;        // 基类实现返回 "base"
    virtual double discount() const = 0;      // 纯虚函数
};

class VipCustomer : public CustomerBase {
public:
    VipCustomer(double discount_rate);
    std::string label() const override;
    double discount() const override;
private:
    double discount_rate_;
};

std::unique_ptr<CustomerBase> make_base();
std::unique_ptr<VipCustomer> make_vip(double discount_rate);

// ===== 模式 4：演示多种模式的自由函数 =====
double compute_discounted_price(const CustomerBase& c, double price);
std::vector<int> doubled_values(const std::vector<int>& v);

// ===== 模式 5：constexpr 静态数据（来自 046） =====
struct Settings {
    static constexpr int    MAX_CUSTOMERS = 1000;
    static constexpr double DEFAULT_DISCOUNT = 0.10;
};

} // namespace summary_ns
