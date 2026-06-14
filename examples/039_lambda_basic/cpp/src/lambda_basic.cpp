#include "hicc_usages/lambda_basic.h"
#include <algorithm>
#include <functional>
namespace hicc_usages::lambda_basic {

int apply_double(int x) {
    auto fn = [](int v) { return v * 2; };
    return fn(x);
}

int apply_square(int x) {
    auto fn = [](int v) { return v * v; };
    return fn(x);
}

int apply_negate(int x) {
    auto fn = [](int v) { return -v; };
    return fn(x);
}

int sum_with_lambda(int a, int b, int c) {
    int total = 0;
    auto adder = [&total](int v) { total += v; };
    adder(a); adder(b); adder(c);
    return total;
}

int count_if_positive(int* values, std::size_t count) {
    if (!values) return 0;
    return static_cast<int>(std::count_if(values, values + count,
        [](int v) { return v > 0; }));
}

Calculator::Calculator() = default;
Calculator::~Calculator() = default;
Calculator* Calculator::create() { return new Calculator(); }
void Calculator::free(Calculator* self) { delete self; }
int Calculator::add(int a, int b) const {
    auto fn = [](int x, int y) { return x + y; };
    return fn(a, b);
}
int Calculator::multiply(int a, int b) const {
    auto fn = [](int x, int y) { return x * y; };
    return fn(a, b);
}
int Calculator::square(int x) const { return apply_square(x); }
int Calculator::factorial(int n) const {
    std::function<int(int)> fact = [&fact](int k) -> int {
        return k <= 1 ? 1 : k * fact(k - 1);
    };
    return fact(n);
}
}
