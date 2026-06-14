#include "hicc_usages/lambda_basic.h"
#include <cassert>
#include <iostream>
int main() {
    using namespace hicc_usages::lambda_basic;
    assert(apply_double(5) == 10);
    assert(apply_square(7) == 49);
    assert(apply_negate(3) == -3);
    assert(sum_with_lambda(10, 20, 30) == 60);

    int vals[] = {-1, 2, -3, 4, 5, -6};
    assert(count_if_positive(vals, 6) == 3);

    Calculator* c = Calculator::create();
    assert(c->add(3, 4) == 7);
    assert(c->multiply(3, 4) == 12);
    assert(c->square(5) == 25);
    assert(c->factorial(5) == 120);
    Calculator::free(c);
    std::cout << "[lambda_basic] C++ test OK" << std::endl;
    return 0;
}
