#pragma once
#include <cstddef>
#include <iostream>
namespace hicc_usages::lambda_basic {

int apply_double(int x);
int apply_square(int x);
int apply_negate(int x);
int sum_with_lambda(int a, int b, int c);
int count_if_positive(int* values, std::size_t count);

class Calculator {
public:
    static Calculator* create();
    static void free(Calculator* self);
    int add(int a, int b) const;
    int multiply(int a, int b) const;
    int square(int x) const;
    int factorial(int n) const;
private:
    Calculator();
    ~Calculator();
};

}  // namespace hicc_usages::lambda_basic
