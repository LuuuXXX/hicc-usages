#pragma once
#include <string>
#include <vector>
#include <memory>
#include <stdexcept>
#include <iostream>

namespace summary_ns {

// ===== Pattern 1: enum class wrapped via int (from 044) =====
enum class CustomerTier : int { Free = 0, Basic = 1, Premium = 2 };

inline int tier_to_int(CustomerTier t) { return static_cast<int>(t); }
inline CustomerTier tier_from_int(int v) {
    if (v < 0 || v > 2) return CustomerTier::Free;
    return static_cast<CustomerTier>(v);
}

// ===== Pattern 2: Class with std::string fields, std::vector, exception on error (combines 006/034/042) =====
class Customer {
public:
    Customer(int id, const std::string& name, CustomerTier tier);

    int id() const;
    std::string name() const;
    CustomerTier tier() const;

    void rename(const std::string& new_name);
    void upgrade(CustomerTier new_tier);

    // Throws std::invalid_argument if amount <= 0
    void charge(int amount);

    // Throws std::out_of_range if idx >= purchases.size()
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

// ===== Pattern 3: Single inheritance virtual method (from 013/015) =====
class CustomerBase {
public:
    CustomerBase() = default;
    virtual ~CustomerBase() = default;
    virtual std::string label() const;        // base impl returns "base"
    virtual double discount() const = 0;      // pure virtual
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

// ===== Pattern 4: Free functions demonstrating several patterns =====
double compute_discounted_price(const CustomerBase& c, double price);
std::vector<int> doubled_values(const std::vector<int>& v);

// ===== Pattern 5: constexpr static data (from 046) =====
struct Settings {
    static constexpr int    MAX_CUSTOMERS = 1000;
    static constexpr double DEFAULT_DISCOUNT = 0.10;
};

} // namespace summary_ns
