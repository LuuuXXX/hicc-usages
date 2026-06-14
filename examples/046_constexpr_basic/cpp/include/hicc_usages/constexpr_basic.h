#pragma once
#include <cstddef>
#include <iostream>
namespace hicc_usages::constexpr_basic {

constexpr int factorial(int n) {
    return n <= 1 ? 1 : n * factorial(n - 1);
}

constexpr int fibonacci(int n) {
    if (n <= 1) return n;
    return fibonacci(n - 1) + fibonacci(n - 2);
}

constexpr int power(int base, int exp) {
    int result = 1;
    for (int i = 0; i < exp; ++i) result *= base;
    return result;
}

int call_factorial(int n);
int call_fibonacci(int n);
int call_power(int base, int exp);
int square_const(int x);

class ConstContainer {
public:
    static ConstContainer* create();
    static void free(ConstContainer* self);
    int get_factorial_5() const;
    int get_fibonacci_10() const;
    int get_power_2_10() const;
    int size() const;
    int at(int idx) const;
private:
    ConstContainer();
    ~ConstContainer();
    static constexpr int SIZE = 5;
    int values_[SIZE];
};

}  // namespace hicc_usages::constexpr_basic
