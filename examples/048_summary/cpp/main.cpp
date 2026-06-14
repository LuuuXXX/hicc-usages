#include "summary.h"
#include <iostream>

int main() {
    using namespace summary_ns;
    auto c = make_customer(1, "alice", static_cast<int>(CustomerTier::Basic));
    std::cout << "c.describe() = " << c->describe() << std::endl;
    c->charge(50);
    c->charge(100);
    std::cout << "c.total_spent() = " << c->total_spent() << std::endl;
    std::cout << "c.purchase_count() = " << c->purchase_count() << std::endl;

    try {
        c->charge(-5);
    } catch (const std::exception& e) {
        std::cout << "charge caught: " << e.what() << std::endl;
    }

    try {
        c->purchase_at(99);
    } catch (const std::exception& e) {
        std::cout << "purchase_at caught: " << e.what() << std::endl;
    }

    auto vip = make_vip(0.20);
    std::cout << "vip.label() = " << vip->label() << std::endl;
    std::cout << "vip.discount() = " << vip->discount() << std::endl;
    std::cout << "discounted price of 100 = "
              << compute_discounted_price(*vip, 100.0) << std::endl;

    std::cout << "MAX_CUSTOMERS = " << Settings::MAX_CUSTOMERS << std::endl;
    std::cout << "DEFAULT_DISCOUNT = " << Settings::DEFAULT_DISCOUNT << std::endl;

    return 0;
}
