#pragma once
#include <iostream>

namespace inline_ns {

inline int square(int x) { return x * x; }
inline int cube(int x) { return x * x * x; }

constexpr inline int factorial(int n) {
    return n <= 1 ? 1 : n * factorial(n - 1);
}

} // namespace inline_ns
