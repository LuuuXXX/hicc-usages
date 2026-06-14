#pragma once

#include <stdexcept>
#include <string>

// Synthesis: combines class with factory/free, exception via hicc::Exception<T>,
// std::string via wrapper, enum-via-int bridge, and a free utility function.

enum class OpKind : int { Add = 0, Sub = 1, Mul = 2, Div = 3 };

class Calculator {
public:
    Calculator(int seed);
    ~Calculator();
    int  base() const;
    int  apply(OpKind op, int x, int y) const; // throws std::runtime_error on Div-by-0
    std::string describe() const;

private:
    int seed_;
};

Calculator* calc_new(int seed);
void calc_free(Calculator* c);

// Top-level utility.
int op_kind_int(int kind);
const char* calc_version();

// Adapter: stable C-string with Calculator lifetime.
// (describe() returns a temporary std::string — c_str() dangles.)
const char* calc_describe_c(const Calculator* c);
