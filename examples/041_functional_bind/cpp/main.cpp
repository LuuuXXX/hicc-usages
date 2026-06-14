#include "functional_bind.h"
#include <iostream>

int main() {
    using namespace functional_bind_ns;

    auto adder = make_adder(10);    // _1 + 10
    std::cout << "adder(5) = " << adder(5) << std::endl;

    auto mult = make_multiplier(3); // _1 * 3
    std::cout << "mult(4) = " << mult(4) << std::endl;

    auto sub = make_subtractor(100); // _1 - 100
    std::cout << "sub(150) = " << sub(150) << std::endl;

    std::cout << "apply_bound(mult, 6) = " << apply_bound(mult, 6) << std::endl;

    auto composed = compose(mult, adder);  // (_1 + 10) * 3
    std::cout << "composed(2) = " << composed(2) << std::endl;

    auto acc = make_accumulator(adder);
    std::cout << "acc.call_and_accumulate(5) = " << acc->call_and_accumulate(5) << std::endl;
    std::cout << "acc.base() = " << acc->base() << std::endl;

    return 0;
}
