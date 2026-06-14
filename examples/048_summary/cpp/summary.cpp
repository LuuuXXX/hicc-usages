#include "summary.h"

namespace summary_ns {

Customer::Customer(int id, const std::string& name, CustomerTier tier)
    : id_(id), name_(name), tier_(tier), total_spent_(0) {}

int Customer::id() const { return id_; }
std::string Customer::name() const { return name_; }
CustomerTier Customer::tier() const { return tier_; }

void Customer::rename(const std::string& new_name) { name_ = new_name; }

void Customer::upgrade(CustomerTier new_tier) {
    if (static_cast<int>(new_tier) < static_cast<int>(tier_)) {
        throw std::invalid_argument("cannot downgrade");
    }
    tier_ = new_tier;
}

void Customer::charge(int amount) {
    if (amount <= 0) {
        throw std::invalid_argument("charge amount must be positive");
    }
    purchases_.push_back(amount);
    total_spent_ += amount;
}

int Customer::purchase_at(size_t idx) const {
    if (idx >= purchases_.size()) {
        throw std::out_of_range("purchase index out of range");
    }
    return purchases_[idx];
}

size_t Customer::purchase_count() const { return purchases_.size(); }
int Customer::total_spent() const { return total_spent_; }
void Customer::add_purchase(int amount) {
    purchases_.push_back(amount);
    total_spent_ += amount;
}

std::string Customer::describe() const {
    std::string tier_name;
    switch (tier_) {
        case CustomerTier::Free:   tier_name = "free";   break;
        case CustomerTier::Basic:  tier_name = "basic";  break;
        case CustomerTier::Premium: tier_name = "premium"; break;
    }
    return "Customer(" + std::to_string(id_) + ", " + name_ + ", " + tier_name + ")";
}

std::unique_ptr<Customer> make_customer(int id, const std::string& name, int tier) {
    return std::unique_ptr<Customer>(new Customer(id, name, tier_from_int(tier)));
}

// ----- CustomerBase -----
std::string CustomerBase::label() const { return "base"; }

// ----- VipCustomer -----
VipCustomer::VipCustomer(double discount_rate) : discount_rate_(discount_rate) {}
std::string VipCustomer::label() const { return "vip"; }
double VipCustomer::discount() const { return discount_rate_; }

std::unique_ptr<CustomerBase> make_base() {
    return std::unique_ptr<VipCustomer>(new VipCustomer(0.05));
}
std::unique_ptr<VipCustomer> make_vip(double discount_rate) {
    return std::unique_ptr<VipCustomer>(new VipCustomer(discount_rate));
}

double compute_discounted_price(const CustomerBase& c, double price) {
    return price * (1.0 - c.discount());
}

std::vector<int> doubled_values(const std::vector<int>& v) {
    std::vector<int> out;
    out.reserve(v.size());
    for (int x : v) out.push_back(x * 2);
    return out;
}

int summary_anchor() { return 48; }

} // namespace summary_ns
