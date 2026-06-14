#include "hicc_usages/constexpr_basic.h"
namespace hicc_usages::constexpr_basic {

int call_factorial(int n) { return factorial(n); }
int call_fibonacci(int n) { return fibonacci(n); }
int call_power(int base, int exp) { return power(base, exp); }
int square_const(int x) {
    constexpr auto sq = [](int v) constexpr { return v * v; };
    return sq(x);
}

ConstContainer::ConstContainer() : values_{1, 2, 3, 4, 5} {}
ConstContainer::~ConstContainer() = default;
ConstContainer* ConstContainer::create() { return new ConstContainer(); }
void ConstContainer::free(ConstContainer* self) { delete self; }
int ConstContainer::get_factorial_5() const { return factorial(5); }
int ConstContainer::get_fibonacci_10() const { return fibonacci(10); }
int ConstContainer::get_power_2_10() const { return power(2, 10); }
int ConstContainer::size() const { return SIZE; }
int ConstContainer::at(int idx) const {
    if (idx < 0 || idx >= SIZE) return -1;
    return values_[idx];
}
}
