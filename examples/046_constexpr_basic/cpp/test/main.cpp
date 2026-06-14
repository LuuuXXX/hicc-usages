#include "hicc_usages/constexpr_basic.h"
#include <cassert>
#include <iostream>
int main() {
    using namespace hicc_usages::constexpr_basic;
    static_assert(factorial(5) == 120, "factorial");
    static_assert(fibonacci(10) == 55, "fibonacci");
    static_assert(power(2, 10) == 1024, "power");

    assert(call_factorial(5) == 120);
    assert(call_fibonacci(10) == 55);
    assert(call_power(2, 10) == 1024);
    assert(square_const(7) == 49);

    ConstContainer* c = ConstContainer::create();
    assert(c->get_factorial_5() == 120);
    assert(c->get_fibonacci_10() == 55);
    assert(c->get_power_2_10() == 1024);
    assert(c->size() == 5);
    assert(c->at(0) == 1);
    assert(c->at(4) == 5);
    ConstContainer::free(c);
    std::cout << "[constexpr_basic] C++ test OK" << std::endl;
    return 0;
}
